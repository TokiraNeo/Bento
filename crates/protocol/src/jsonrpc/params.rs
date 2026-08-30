/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::tool::ToolDefinition;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Host -> Bento
/// ```json
/// {
///   "protocol_version": "2026-07-28",
///   "host_name": "MyHost",
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostHelloParam {
    pub protocol_version: String,
    pub host_name: String,
}

/// ```json
/// {
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostReadyParam {}

/// ```json
/// {
///   "tools": [
///     {"name": "...", "description": "...", "input_schema": {...}, "risk": "medium", "domain": "...", "tags": ["..."], "example": "..."},
///   ]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRegisterParam {
    pub tools: Vec<ToolDefinition>,
}

/// ```json
/// {
///   "tool_name": "...",
///   "arguments": {...}
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallParam {
    pub tool_name: String,
    pub arguments: Value,
}
