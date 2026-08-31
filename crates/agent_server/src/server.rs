/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod mcp;

use crate::AgentServerConfig;
use crate::tool_query::ToolQuerySink;
use axum::ServiceExt;
use mcp::AgentMcpServer;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::{StreamableHttpServerConfig, StreamableHttpService};
use std::borrow::Cow;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

pub struct AgentServer {
    config: AgentServerConfig,

    tool_query_sink: Arc<dyn ToolQuerySink>,

    cancel_signal: CancellationToken,
}

impl AgentServer {
    pub fn new(config: &AgentServerConfig, tool_query: Arc<dyn ToolQuerySink>) -> Self {
        Self {
            config: config.clone(),
            tool_query_sink: tool_query,
            cancel_signal: CancellationToken::new(),
        }
    }

    pub async fn run(&self) -> Result<(), Cow<'static, str>> {
        let ct = self.cancel_signal.clone();

        let sink = self.tool_query_sink.clone();

        let service = StreamableHttpService::new(
            move || Ok(AgentMcpServer::new(sink.clone())),
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig::default().with_cancellation_token(ct.child_token()),
        );

        let router = axum::Router::new().nest_service("/mcp", service);

        let listener = TcpListener::bind((self.config.host.as_str(), self.config.port))
            .await
            .map_err(|err| Err(Cow::Owned(err.to_string())))?;

        axum::serve(listener, router)
            .with_graceful_shutdown(async move { ct.cancelled().await })
            .await
            .map_err(|err| Cow::Owned(err.to_string()))
    }

    pub fn stop(&self) {
        self.cancel_signal.cancel();
    }
}
