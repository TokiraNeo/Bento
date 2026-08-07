/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::session::HostRuntimeIdentity;
use bento_protocol::tool::{ToolCallResult, ToolDefinition};

pub enum HostEvent {
Connected {
    namespace: String,
    session_id: String,
    identity: HostRuntimeIdentity,
},
Disconnected {
    namespace: String,
},
ToolsRegistered {
    namespace: String,
    tools: Vec<ToolDefinition>,
},
Ready {
    namespace: String,
},
ToolResult(ToolCallResult),
}

pub struct HostHandle {}
