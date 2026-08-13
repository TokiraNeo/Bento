/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bento_protocol::dispatch::OutboundFrame;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::tungstenite::Message;

#[derive(Clone)]
pub(super) struct HostHandler(pub mpsc::Sender<Message>);

impl HostHandler {
    pub async fn send(&self, frame: OutboundFrame) {
        let msg = Message::Text(serde_json::to_string(&frame).unwrap_or_default().into());
        let _ = self.0.send(msg).await;
    }

    pub async fn send_pong(&self, payload: tokio_tungstenite::tungstenite::Bytes) {
        let _ = self.0.send(Message::Pong(payload)).await;
    }
}

type HostHandlerMap = HashMap<String, HostHandler>;

pub(super) type HostHandlerRegistry = Arc<Mutex<HostHandlerMap>>;

#[derive(Clone)]
pub enum HostEvent {
    HostConnected {
        session_id: String,
    },
    HostHelloed {
        namespace: String,
    },
    HostRegistered {
        session_id: String,
        tool_count: usize,
    },
    HostReady {
        session_id: String,
    },
    HostClosed {
        session_id: String,
    },
}

#[derive(Clone)]
pub(super) struct HostEventBus {
    sender: broadcast::Sender<HostEvent>,
}

impl HostEventBus {
    pub fn new() -> Self {
        let (sender, _receiver) = broadcast::channel(256);

        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<HostEvent> {
        self.sender.subscribe()
    }

    pub fn emit(&self, event: HostEvent) {
        let _ = self.sender.send(event);
    }
}
