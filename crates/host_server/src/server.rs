/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::collections::HashMap;
use crate::session::HostSession;
use crate::event::{HostEvent, HostHandle};
use tokio::sync::mpsc;
use crate::config::HostServerConfig;

pub struct HostServer {
    hosts: HashMap<String, HostSession>,

    pub events: mpsc::Receiver<HostEvent>,
    pub handle: HostHandle,
}

impl HostServer {
    pub fn new(config: HostServerConfig) -> Self {
        let (tx, rx) = mpsc::channel(config.buffer_size);

        HostServer {
            hosts: HashMap::new(),
            events: rx,
            handle: HostHandle {},
        }
    }
}
