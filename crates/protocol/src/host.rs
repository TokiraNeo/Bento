/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Host -> Bento
/// ```json
/// {
///   "protocol_version": "2026-07-28",
///   "host_name": "my_host",
///   "host_version": "1.0.0",
///   "token": "my_secret_token",
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostHello {
    pub protocol_version: String,
    pub host_name: String,
    pub host_version: String,

    pub token: String,

    #[serde(default)]
    pub metadata: Value,
}

/// Bento -> Host
/// ```json
/// {
///   "session_id": "unique_session_id",
///   "namespace": "unique_namespace",
///   "protocol_version": "2026-07-28",
///   "bento_version": "1.0.0",
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostWelcome {
    pub session_id: String,
    pub namespace: String,
    pub protocol_version: String,
    pub bento_version: String,
}
