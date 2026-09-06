/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use bento_host_server::HostMeta;
use tokio::sync::broadcast;

#[derive(Clone)]
pub enum CoreEvent {
    HostsChanged { hosts: Vec<HostMeta> },
}

#[derive(Clone)]
pub(crate) struct CoreEventBus {
    sender: broadcast::Sender<CoreEvent>,
}

impl CoreEventBus {
    pub fn new() -> Self {
        let (sender, _receiver) = broadcast::channel(256);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<CoreEvent> {
        self.sender.subscribe()
    }

    pub fn emit(&self, event: CoreEvent) {
        let _ = self.sender.send(event);
    }
}
