/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::{
    event::{HostEvent, HostEventBus, HostHandler, HostHandlerRegistry},
    session::{HostSession, HostSessionState},
};
use bento_protocol::dispatch::{InboundFrame, OutboundFrame, parse_inbound_frame};
use bento_protocol::jsonrpc::JsonRpcResponse;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, watch},
};
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::Message};
use uuid::Uuid;

pub(super) async fn listen_connection(
    listener: TcpListener,
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
                        let session_id = Uuid::new_v4().to_string();
                        let clone_bus = bus.clone();
                        let clone_registry = registry.clone();

                        tokio::spawn(async move {
                            handle_connection(tcp, session_id, clone_bus, clone_registry).await;
                        });
                    }

                    Err(_) => {}
                }
            }
        }
    }
}

async fn handle_connection(
    tcp: TcpStream,
    session_id: String,
    bus: HostEventBus,
    registry: HostHandlerRegistry,
) {
    let ws: WebSocketStream<TcpStream> = match accept_async(tcp).await {
        Ok(ws) => ws,
        Err(_) => {
            todo!("Log Error here.");
        }
    };

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
                //todo!("Log Info here.");
                break;
            }
        }
    });

    while let Some(Ok(msg)) = reader.next().await {
        if !handle_message(&mut session, msg, &bus).await {
            //todo!("Log Info here.");
            break;
        }
    }

    let _ = write_task.await;

    // Close session and broadcast closed event.
    session.state = HostSessionState::Closed;
    bus.emit(HostEvent::HostClosed { session_id });
}

async fn handle_message(session: &mut HostSession, msg: Message, bus: &HostEventBus) -> bool {
    match msg {
        Message::Text(text) => {
            match parse_inbound_frame(text.as_str()) {
                Ok(frame) => {
                    handle_inbound_frame(frame, session, bus).await;
                }
                Err(err) => {
                    //todo!("Log Error here");

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
        InboundFrame::Request(request) => {}
        InboundFrame::Notification(notification) => {}
    }
}
