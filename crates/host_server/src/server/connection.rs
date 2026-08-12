/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::{
    event::{HostEvent, HostEventBus, HostHandler, HostHandlerRegistry},
    session::{HostSession, HostSessionState},
};
use bento_protocol::{
    commands::{host_command, tool_command},
    dispatch::{InboundFrame, OutboundFrame, parse_inbound_frame},
    jsonrpc::JsonRpcResponse,
};
use futures_util::{SinkExt, StreamExt};
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
use uuid::Uuid;

#[tracing::instrument(skip(listener, token, bus, registry, shutdown_signal))]
pub(super) async fn listen_connection(
    listener: TcpListener,
    token: String,
    bus: HostEventBus,
    registry: HostHandlerRegistry,
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
                        let clone_registry = registry.clone();

                        tokio::spawn(async move {
                            handle_connection(tcp, clone_token, clone_bus, clone_registry).await;
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

#[tracing::instrument(skip(tcp, token, bus, registry))]
async fn handle_connection(
    tcp: TcpStream,
    token: String,
    bus: HostEventBus,
    registry: HostHandlerRegistry,
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

    let session_id = Uuid::new_v4().to_string();

    let (mut writer, mut reader) = ws.split();

    let (sender, mut receiver) = mpsc::channel::<Message>(1024);

    let handler = HostHandler(sender);

    // Insert new Handler for new host session.
    registry
        .lock()
        .unwrap()
        .insert(session_id.clone(), handler.clone());

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
        if !handle_message(&mut session, msg, &bus).await {
            warn!("Failed to handle message");
            break;
        }
    }

    let _ = write_task.await;

    // Close session and broadcast closed event.
    session.state = HostSessionState::Closed;
    bus.emit(HostEvent::HostClosed { session_id });
}

#[tracing::instrument(skip(session, msg, bus, registry))]
async fn handle_message(session: &mut HostSession, msg: Message, bus: &HostEventBus) -> bool {
    match msg {
        Message::Text(text) => {
            match parse_inbound_frame(text.as_str()) {
                Ok(frame) => {
                    handle_inbound_frame(frame, session, bus).await;
                }
                Err(err) => {
                    warn!("Failed to parse inbound frame: {:?}", err);

                    let response = JsonRpcResponse::new_error(String::new(), err);

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

async fn handle_inbound_frame(frame: InboundFrame, session: &mut HostSession, bus: &HostEventBus) {
    match frame {
        InboundFrame::Request(request) => {
            let method = request.method.as_str();

            match (session.state, method) {
                (HostSessionState::Connecting, host_command::HOST_HELLO) => {}
                (HostSessionState::Helloed, tool_command::TOOLS_REGISTER) => {}
                _ => {}
            }
        }
        InboundFrame::Notification(notification) => {
            let method = notification.method.as_str();

            match (session.state, method) {
                (HostSessionState::Registered, host_command::HOST_READY) => {}
                _ => {}
            }
        }
    }
}
