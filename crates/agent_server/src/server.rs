/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::AgentServerConfig;
use crate::tool_query::ToolQuerySink;
use std::sync::Arc;

pub struct AgentServer {
    config: AgentServerConfig,

    tool_query_sink: Arc<dyn ToolQuerySink>,
}

impl AgentServer {
    pub fn new(config: AgentServerConfig, tool_query: Arc<dyn ToolQuerySink>) -> Self {
        Self {
            config,
            tool_query_sink: tool_query,
        }
    }
}
