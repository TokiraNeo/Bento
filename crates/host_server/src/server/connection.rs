/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::namespace::HostNamespaceRegistry;
use crate::request_task::RequestTaskManager;
use crate::tool_index::ToolIndexSink;
use crate::{
    event::{HostEvent, HostEventBus, HostHandler, HostHandlerRegistry},
    session::{HostSession, HostSessionState},
};
use bento_protocol::error::ErrorCode;
use bento_protocol::jsonrpc::params::{HostHelloParams, HostReadyParams, ToolRegisterParams};
use bento_protocol::jsonrpc::results::{HostWelcomeResult, ToolRegisterResult};
use bento_protocol::jsonrpc::templates::{
    TJsonRpcResponse, from_notification, from_request, into_response,
};
use bento_protocol::jsonrpc::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use bento_protocol::versions::{BENTO_VERSION, MCP_PROTOCOL_VERSION};
use bento_protocol::{
    commands::{host_command, tool_command},
    dispatch::{InboundFrame, OutboundFrame, parse_frame},
};
use bento_utility::generate_uuid_simple;
use futures_util::{SinkExt, StreamExt};
use std::borrow::Cow;
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, watch},
};
use tokio_tungstenite::{
    WebSocketStream, accept_hdr_async,
    tungstenite::http::StatusCode,
    tungstenite::{
        Message,
        handshake::server::{ErrorResponse, Request, Response},
    },
};
use tracing::{error, warn};

#[tracing::instrument(skip(
    listener,
    token,
    bus,
    handlers,
    namespaces,
    request_manager,
    index_sink,
    shutdown_signal
))]
pub(super) async fn listen_connection(
    listener: TcpListener,
    token: String,
    bus: HostEventBus,
    handlers: HostHandlerRegistry,
    namespaces: HostNamespaceRegistry,
    request_manager: RequestTaskManager,
    index_sink: Arc<dyn ToolIndexSink>,
    mut shutdown_signal: watch::Receiver<bool>,
) {
    loop {
        tokio::select! {
            _ = shutdown_signal.changed() => {
                break;
            }

            accepted = listener.accept() => {
                match accepted {
                    Ok((tcp, _)) => {
                        let clone_token = token.clone();
                        let clone_bus = bus.clone();
                        let clone_handlers = handlers.clone();
                        let clone_namespaces = namespaces.clone();
                        let clone_request_manager = request_manager.clone();
                        let clone_index_sink = index_sink.clone();

                        tokio::spawn(async move {
                            handle_connection(tcp, clone_token, clone_bus, clone_handlers, clone_namespaces, clone_request_manager,
                            clone_index_sink).await;
                        });
                    }

                    Err(_) => {
                        error!("Error accepting tcp connection");
                    }
                }
            }
        }
    }
}

#[tracing::instrument(skip(tcp, token, bus, handlers, namespaces, request_manager, index_sink))]
async fn handle_connection(
    tcp: TcpStream,
    token: String,
    bus: HostEventBus,
    handlers: HostHandlerRegistry,
    namespaces: HostNamespaceRegistry,
    request_manager: RequestTaskManager,
    index_sink: Arc<dyn ToolIndexSink>,
) {
    let ws: WebSocketStream<TcpStream> = match accept_hdr_async(
        tcp,
        |request: &Request, response: Response| -> Result<Response, ErrorResponse> {
            let got_token = request
                .headers()
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
                .map(str::trim)
                .unwrap_or_default();

            if !token.is_empty() && got_token == token {
                Ok(response)
            } else {
                Err(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Some("unauthorized".to_string()))
                    .unwrap())
            }
        },
    )
    .await
    {
        Ok(ws) => ws,
        Err(_) => {
            error!("Failed to accept websocket stream.");
            return;
        }
    };

    let session_id = generate_uuid_simple();

    let (mut writer, mut reader) = ws.split();

    let (sender, mut receiver) = mpsc::channel::<Message>(1024);

    let handler = HostHandler::new(sender);

    // Insert new Handler for new host session.
    handlers.register(session_id.clone(), handler.clone());

    let mut session = HostSession::new(session_id.clone(), handler);

    // Broadcast new host session event.
    bus.emit(HostEvent::HostConnected {
        session_id: session_id.clone(),
    });

    let write_task = tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            if writer.send(msg).await.is_err() {
                error!("Failed to send message");
                break;
            }
        }
    });

    while let Some(Ok(msg)) = reader.next().await {
        if !handle_message(
            &mut session,
            msg,
            &bus,
            &namespaces,
            &request_manager,
            &index_sink,
        )
        .await
        {
            warn!("Failed to handle message");
            break;
        }
    }

    // 只有sender全部drop后，receiver的recv()才会返回None，所以这里先确保sender的两份副本drop: session、handlers

    request_manager.cancel_all_for_session(&session_id);
    handlers.remove(&session_id); // 释放sender

    if let Err(err) = index_sink.remove(&session_id).await {
        warn!("Failed to remove tools for session {}: {}", session_id, err);
    }

    namespaces.release(session.namespace.clone());

    session.state = HostSessionState::Closed;
    drop(session); // 释放sender

    let _ = write_task.await;

    bus.emit(HostEvent::HostClosed { session_id });
}

#[tracing::instrument(skip(session, msg, bus, namespaces, request_manager, index_sink))]
async fn handle_message(
    session: &mut HostSession,
    msg: Message,
    bus: &HostEventBus,
    namespaces: &HostNamespaceRegistry,
    request_manager: &RequestTaskManager,
    index_sink: &Arc<dyn ToolIndexSink>,
) -> bool {
    match msg {
        Message::Text(text) => {
            match parse_frame(text.as_str()) {
                Ok(frame) => {
                    handle_inbound_frame(
                        frame,
                        session,
                        bus,
                        namespaces,
                        request_manager,
                        index_sink,
                    )
                    .await;
                }
                Err(err) => {
                    error!("Failed to parse inbound frame: {:?}", err);

                    let id = generate_uuid_simple();
                    let response = JsonRpcResponse::new_error(id, err);

                    session
                        .handler
                        .send(OutboundFrame::Response(response))
                        .await;
                }
            }

            true
        }
        Message::Ping(payload) => {
            session.handler.send_pong(payload).await;
            true
        }
        Message::Pong(_) => true,
        Message::Binary(_) => false,
        Message::Close(_) => false,
        Message::Frame(_) => false,
    }
}

#[tracing::instrument(skip(frame, session, bus, namespaces, request_manager, index_sink))]
async fn handle_inbound_frame(
    frame: InboundFrame,
    session: &mut HostSession,
    bus: &HostEventBus,
    namespaces: &HostNamespaceRegistry,
    request_manager: &RequestTaskManager,
    index_sink: &Arc<dyn ToolIndexSink>,
) {
    match frame {
        InboundFrame::Request(request) => match (session.state, request.method.as_str()) {
            (HostSessionState::Connecting, host_command::HOST_HELLO) => {
                handle_host_hello(session, request, bus, namespaces).await;
            }
            (HostSessionState::Helloed, tool_command::TOOLS_REGISTER) => {
                handle_tool_register(session, request, bus, index_sink).await;
            }
            (state, method) => {
                warn!(
                    "Mismatch state and method in request: {:?} - {}",
                    state, method
                );
            }
        },

        InboundFrame::Notification(notification) => {
            match (session.state, notification.method.as_str()) {
                (HostSessionState::Registered, host_command::HOST_READY) => {
                    handle_host_ready(session, notification, bus, index_sink).await;
                }
                (state, method) => {
                    warn!(
                        "Mismatch state and method in notification: {:?} - {}",
                        state, method
                    );
                }
            }
        }

        InboundFrame::Response(response) => {
            let id = response.id.clone();
            if !request_manager.response(response) {
                warn!("Unexpected response, no pending for id: {}", id);
            }
        }
    }
}

#[tracing::instrument(skip(session, request, bus, namespaces))]
async fn handle_host_hello(
    session: &mut HostSession,
    request: JsonRpcRequest,
    bus: &HostEventBus,
    namespaces: &HostNamespaceRegistry,
) {
    match from_request::<HostHelloParams>(request) {
        Ok(rpc) => {
            // Register new namespace for host
            let namespace = namespaces.register(rpc.params.host_name, session.session_id.clone());

            session.state = HostSessionState::Helloed;
            session.namespace = namespace.clone();

            // Broadcast host.hello event
            bus.emit(HostEvent::HostHelloed {
                namespace: namespace.clone(),
            });

            // response
            let response = TJsonRpcResponse::<HostWelcomeResult> {
                jsonrpc: rpc.jsonrpc,
                id: rpc.id,
                result: Some(HostWelcomeResult {
                    namespace,
                    protocol_version: MCP_PROTOCOL_VERSION.into(),
                    bento_version: BENTO_VERSION.into(),
                }),
                error: None,
            };

            match into_response(response) {
                Ok(resp) => session.handler.send(OutboundFrame::Response(resp)).await,
                Err(err) => {
                    error!("Failed to cast response: {:?}", err);
                }
            }
        }
        Err(err) => {
            error!("Failed to cast request: {:?}", err);
        }
    }
}

#[tracing::instrument(skip(session, request, bus, index_sink))]
async fn handle_tool_register(
    session: &mut HostSession,
    request: JsonRpcRequest,
    bus: &HostEventBus,
    index_sink: &Arc<dyn ToolIndexSink>,
) {
    match from_request::<ToolRegisterParams>(request) {
        Ok(rpc) => {
            let tools = rpc.params.tools;

            // validate tools
            let invalid: Vec<String> = tools
                .iter()
                .filter_map(|tool| match tool.validate() {
                    Ok(()) => None,
                    Err(e) => Some(format!("{}: {}", tool.name, e)),
                })
                .collect();

            if !invalid.is_empty() {
                let response = TJsonRpcResponse::<ToolRegisterResult> {
                    jsonrpc: rpc.jsonrpc,
                    id: rpc.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: ErrorCode::InvalidToolArgs.code(),
                        message: Cow::Borrowed("Found invalid tools."),
                        payload: Some(serde_json::to_value(&invalid).unwrap()),
                    }),
                };

                match into_response(response) {
                    Ok(resp) => session.handler.send(OutboundFrame::Response(resp)).await,
                    Err(err) => {
                        error!("Failed to cast response: {:?}", err);
                    }
                }
            } else {
                let response = match index_sink
                    .replace(&session.session_id, &session.namespace, tools)
                    .await
                {
                    Ok(count) => {
                        session.state = HostSessionState::Registered;

                        // Broadcast host registered event
                        bus.emit(HostEvent::HostRegistered {
                            namespace: session.namespace.clone(),
                            tool_count: count,
                        });

                        TJsonRpcResponse::<ToolRegisterResult> {
                            jsonrpc: rpc.jsonrpc,
                            id: rpc.id,
                            result: Some(ToolRegisterResult { count }),
                            error: None,
                        }
                    }

                    Err(err) => {
                        error!("Failed to index tools: {:?}", err);

                        TJsonRpcResponse::<ToolRegisterResult> {
                            jsonrpc: rpc.jsonrpc,
                            id: rpc.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: ErrorCode::InternalError.code(),
                                message: Cow::Borrowed("Failed to index tools."),
                                payload: None,
                            }),
                        }
                    }
                };

                match into_response(response) {
                    Ok(resp) => session.handler.send(OutboundFrame::Response(resp)).await,
                    Err(err) => {
                        error!("Failed to cast response: {:?}", err);
                    }
                }
            }
        }

        Err(err) => {
            error!("Failed to cast request: {:?}", err);
        }
    }
}

#[tracing::instrument(skip(session, notification, bus, index_sink))]
async fn handle_host_ready(
    session: &mut HostSession,
    notification: JsonRpcNotification,
    bus: &HostEventBus,
    index_sink: &Arc<dyn ToolIndexSink>,
) {
    match from_notification::<HostReadyParams>(notification) {
        Ok(_rpc) => {
            session.state = HostSessionState::Ready;

            if let Err(err) = index_sink.ready(&session.session_id).await {
                warn!(
                    "Failed to mark session {} ready in tool index: {}",
                    session.session_id, err
                );
            }

            bus.emit(HostEvent::HostReady {
                session_id: session.session_id.clone(),
            });
        }
        Err(err) => {
            error!("Failed to cast notification: {:?}", err);
        }
    }
}
