/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;


/// Host -> Bento
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostHello {
    pub protocol_version: String,
    pub host_name: String,
    pub host_version: String,

    pub host_id: String,

    pub token: String,

    #[serde(default)]
    pub metadata: Value,
}

/// Bento -> Host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostWelcome {
    pub session_id: String,
    pub namespace: String,
    pub protocol_version: String,
    pub bento_version: String,
}
