/*
 * ---- Bento ----
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
pub(super) struct HostHandler(mpsc::Sender<Message>);

impl HostHandler {
    pub fn new(sender: mpsc::Sender<Message>) -> Self {
        Self(sender)
    }

    pub async fn send(&self, frame: OutboundFrame) {
        let msg = Message::Text(serde_json::to_string(&frame).unwrap_or_default().into());
        let _ = self.0.send(msg).await;
    }

    pub async fn send_pong(&self, payload: tokio_tungstenite::tungstenite::Bytes) {
        let _ = self.0.send(Message::Pong(payload)).await;
    }
}

type HostHandlerMap = HashMap<String, HostHandler>;

// pub(super) type HostHandlerRegistry = Arc<Mutex<HostHandlerMap>>;

#[derive(Clone)]
pub(super) struct HostHandlerRegistry {
    handlers: Arc<Mutex<HostHandlerMap>>,
}

impl HostHandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, session_id: String, handler: HostHandler) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(session_id, handler);
    }

    pub fn remove(&self, session_id: &str) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.remove(session_id);
    }

    pub fn get(&self, session_id: &str) -> Option<HostHandler> {
        let handlers = self.handlers.lock().unwrap();
        handlers.get(session_id).cloned()
    }
}

#[derive(Clone)]
pub enum HostEvent {
    HostConnected {
        session_id: String,
    },
    HostHelloed {
        namespace: String,
    },
    HostRegistered {
        namespace: String,
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
