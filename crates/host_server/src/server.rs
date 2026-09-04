/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod connection;

use crate::config::HostServerConfig;
use crate::event::{HostEvent, HostEventBus, HostHandlerRegistry};
use crate::namespace::HostNamespaceRegistry;
use crate::request_task::{RequestOutcome, RequestTask, RequestTaskManager};
use crate::tool_index::ToolIndexSink;
use bento_protocol::commands::tool_command;
use bento_protocol::dispatch::OutboundFrame;
use bento_protocol::jsonrpc::params::ToolCallParam;
use bento_protocol::jsonrpc::results::ToolCallResult;
use bento_protocol::jsonrpc::templates::{TJsonRpcRequest, into_request};
use bento_protocol::jsonrpc::{JsonRpcRequest, JsonRpcResponse};
use bento_protocol::versions::JSON_RPC_VERSION;
use bento_utility::generate_uuid_simple;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, oneshot};
use tokio_util::sync::CancellationToken;

/// ```
/// 宿主                                Hub
/// │                                     │
/// │──── WS connect ───────────────────► │  [Connecting]
/// │                                     │
/// │──── host.hello (request) ─────────► │  认证
/// │◄─── host.welcome (response) ─────── │  分配 namespace  [Helloed]
/// │                                     │
/// │──── tools.register (request) ─────► │  宿主主动注册
/// │◄─── tools.registered (response) ─── │  确认  [Registered]
/// │                                     │
/// │──── host.ready (notification) ─────►│  [Ready]
/// │                                     │
/// │◄─── tool.call (request) ─────────── │  Hub 下发调用（就绪后）
/// │──── tool.result (response) ────────►│
/// │                                     │
/// │──── WS disconnect ────────────────► │  [Closed]
/// ```
pub struct HostServer {
    config: HostServerConfig,

    /// A map of session_id to HostHandler
    handlers: HostHandlerRegistry,

    /// A map of namespace to session_id
    namespaces: HostNamespaceRegistry,

    /// Event bus for host events
    bus: HostEventBus,

    request_manager: RequestTaskManager,

    tool_index_sink: Arc<dyn ToolIndexSink>,

    /// Shut down signal for worker
    shutdown: CancellationToken,
}

impl HostServer {
    pub fn new(config: &HostServerConfig, index_sink: Arc<dyn ToolIndexSink>) -> Self {
        Self {
            config: config.clone(),
            handlers: HostHandlerRegistry::new(),
            namespaces: HostNamespaceRegistry::new(),
            bus: HostEventBus::new(),
            request_manager: RequestTaskManager::new(),
            tool_index_sink: index_sink,
            shutdown: CancellationToken::new(),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<HostEvent> {
        self.bus.subscribe()
    }

    pub async fn run(&self) -> Result<(), Cow<'static, str>> {
        let listener = TcpListener::bind((self.config.host.as_str(), self.config.port))
            .await
            .map_err(|err| Cow::Owned(err.to_string()))?;

        let clone_token = self.config.token.clone();
        let clone_bus = self.bus.clone();
        let clone_handlers = self.handlers.clone();
        let clone_namespace = self.namespaces.clone();
        let clone_request_manager = self.request_manager.clone();
        let clone_index_sink = self.tool_index_sink.clone();
        let shutdown = self.shutdown.child_token();

        connection::listen_connection(
            listener,
            clone_token,
            clone_bus,
            clone_handlers,
            clone_namespace,
            clone_request_manager,
            clone_index_sink,
            shutdown,
        )
        .await;

        Ok(())
    }

    pub fn stop(&self) {
        self.shutdown.cancel();
    }

    async fn send_to_host(
        &self,
        session_id: String,
        frame: OutboundFrame,
    ) -> Result<(), Cow<'static, str>> {
        let handler = match self.handlers.get(&session_id) {
            Some(h) => h,
            None => return Err(Cow::Borrowed("Session Handler not found.")),
        };

        handler.send(frame).await;

        Ok(())
    }

    async fn request_to_host(
        &self,
        namespace: String,
        request: JsonRpcRequest,
        timeout: Duration,
    ) -> Result<JsonRpcResponse, Cow<'static, str>> {
        let session_id = self.namespaces.session_id(namespace);

        match session_id {
            Some(s) => {
                let (sender, receiver) = oneshot::channel::<RequestOutcome>();

                let id = request.id.clone();

                self.request_manager.register(RequestTask {
                    id: id.clone(),
                    session_id: s.clone(),
                    responder: sender,
                });

                if let Err(err) = self.send_to_host(s, OutboundFrame::Request(request)).await {
                    return Err(err);
                }

                match tokio::time::timeout(timeout, receiver).await {
                    Ok(Ok(RequestOutcome::Response(resp))) => Ok(resp),

                    Ok(Ok(RequestOutcome::Canceled)) => {
                        Err(Cow::Borrowed("Request Task Canceled."))
                    }

                    Ok(Err(_)) => {
                        self.request_manager.cancel(&id);
                        Err(Cow::Borrowed("Request Task Failed."))
                    }

                    Err(_) => {
                        self.request_manager.cancel(&id);
                        Err(Cow::Borrowed("Request Task Timeout."))
                    }
                }
            }
            None => Err(Cow::Borrowed("Session not found.")),
        }
    }

    pub async fn call_tool(
        &self,
        namespace: String,
        params: ToolCallParam,
        timeout: Duration,
    ) -> Result<ToolCallResult, Cow<'static, str>> {
        let request = into_request(TJsonRpcRequest::<ToolCallParam> {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: generate_uuid_simple(),
            method: tool_command::TOOL_CALL.to_string(),
            params,
        })
        .map_err(|err| err.message)?;

        let response = self.request_to_host(namespace, request, timeout).await?;

        if let Some(err) = response.error {
            return Err(err.message);
        }

        match response.result {
            Some(result) => serde_json::from_value::<ToolCallResult>(result)
                .map_err(|err| Cow::Owned(err.to_string())),

            None => Err(Cow::Borrowed("tool.result missing result")),
        }
    }
}

impl Drop for HostServer {
    fn drop(&mut self) {
        self.stop();
    }
}
