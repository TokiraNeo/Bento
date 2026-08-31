/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use bento_agent_server::AgentServerConfig;
use bento_host_server::HostServerConfig;
use bento_tool_rag::ToolRagConfig;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    /// mcp protocol version
    pub protocol_version: String,
    pub host_server: HostServerConfig,
    pub agent_server: AgentServerConfig,
    pub tool_rag: ToolRagConfig,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            protocol_version: "2026-07-28".into(),
            host_server: HostServerConfig::default(),
            agent_server: AgentServerConfig::default(),
            tool_rag: ToolRagConfig::default(),
        }
    }
}

impl CoreConfig {
    pub fn read(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(text) => serde_json::from_str(&text).unwrap_or_default(),

            Err(_) => {
                let config = CoreConfig::default();
                Self::write(path, &config);
                config
            }
        }
    }

    pub fn write(path: &Path, config: &CoreConfig) {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if let Ok(text) = serde_json::to_string_pretty(config) {
            let _ = std::fs::write(path, text);
        }
    }
}
