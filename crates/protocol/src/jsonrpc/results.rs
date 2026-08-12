/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::tool::ToolCallContent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResult {}

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
pub struct HostWelcomeResult {
    pub session_id: String,
    pub namespace: String,
    pub protocol_version: String,
    pub bento_version: String,
}

/// ```json
/// {
///   "count": 1,
///   "error": "..."
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRegisterResult {
    pub count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// ```json
/// {
///   "call_id": "...",
///   "content": [
///     { "type": "text", "text": "..."}
///   ],
///   "is_error": false
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub call_id: String,
    pub content: Vec<ToolCallContent>,

    #[serde(default)]
    pub is_error: bool,
}
