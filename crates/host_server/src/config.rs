/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct HostServerConfig {
    pub host: String,
    pub port: u16,
    pub token: String,
    pub buffer_size: usize,
}

impl Default for HostServerConfig {
    fn default() -> Self {
        Self {
            host: "".into(),
            port: 2483,
            token: "".into(),
            buffer_size: 256,
        }
    }
}
