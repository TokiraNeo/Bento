/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use bento_agent_server::config::AgentServerConfig;
use bento_host_server::config::HostServerConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    /// mcp protocol version
    pub protocol_version: String,
    pub host_server: HostServerConfig,
    pub agent_server: AgentServerConfig,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            protocol_version: "2026-07-28".into(),
            host_server: HostServerConfig::default(),
            agent_server: AgentServerConfig::default(),
        }
    }
}
