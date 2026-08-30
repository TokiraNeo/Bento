/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::tool::ToolCallContent;
use serde::{Deserialize, Serialize};

/// Bento -> Host
/// ```json
/// {
///   "namespace": "blender#1",
///   "protocol_version": "2026-07-28",
///   "bento_version": "1.0.0",
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostWelcomeResult {
    pub namespace: String,
    pub protocol_version: String,
    pub bento_version: String,
}

/// ```json
/// {
///   "count": 1,
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRegisterResult {
    pub count: usize,
}

/// ```json
/// {
///   "content": [
///     { "type": "text", "text": "..."}
///   ],
///   "is_error": false
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub content: Vec<ToolCallContent>,

    #[serde(default)]
    pub is_error: bool,
}
