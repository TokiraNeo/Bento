/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod connection;

use crate::config::HostServerConfig;
use crate::event::{HostEvent, HostEventBus, HostHandlerRegistry};
use bento_protocol::dispatch::OutboundFrame;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, watch};

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
    registry: HostHandlerRegistry,

    /// Event bus for host events
    bus: HostEventBus,

    /// Shut down signal for worker
    shutdown_sender: watch::Sender<bool>,
    shutdown_receiver: watch::Receiver<bool>,
}

impl HostServer {
    pub fn new(config: HostServerConfig) -> Self {
        let (sender, receiver) = watch::channel(false);

        Self {
            config,
            registry: Arc::new(Mutex::default()),
            bus: HostEventBus::new(),
            shutdown_sender: sender,
            shutdown_receiver: receiver,
        }
    }

    pub fn subcribe(&self) -> broadcast::Receiver<HostEvent> {
        self.bus.subscribe()
    }

    pub async fn run(&self) -> Result<(), String> {
        let listener = TcpListener::bind((self.config.host.as_str(), self.config.port))
            .await
            .map_err(|err| err.to_string())?;

        let clone_token = self.config.token.clone();
        let clone_bus = self.bus.clone();
        let clone_registry = self.registry.clone();
        let shutdown_signal = self.shutdown_receiver.clone();

        tokio::spawn(async move {
            connection::listen_connection(
                listener,
                clone_token,
                clone_bus,
                clone_registry,
                shutdown_signal,
            )
            .await;
        });

        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.shutdown_sender
            .send(true)
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    pub async fn send_to_host(
        &self,
        session_id: String,
        frame: OutboundFrame,
    ) -> Result<(), String> {
        let map = self.registry.lock().unwrap();

        let handler = map.get(&session_id).clone();

        if let None = handler {
            return Err("Session Handler not found.".into());
        }

        handler.unwrap().send(frame).await;

        Ok(())
    }
}
