/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::config::CoreConfig;
use crate::sinks::{RagIndexSink, RagQuerySink};
use bento_agent_server::AgentServer;
use bento_host_server::HostServer;
use bento_tool_rag::ToolRagEngine;
use std::sync::Arc;

pub(super) struct CoreEngine {
    protocol_version: String,

    tool_engine: Arc<ToolRagEngine>,
    host_server: Arc<HostServer>,
    agent_server: Arc<AgentServer>,
}

impl CoreEngine {
    pub fn new(config: CoreConfig) -> Self {
        let tool_engine = Arc::new(ToolRagEngine::new(&config.tool_rag));

        let tool_index_sink = Arc::new(RagIndexSink::new(tool_engine.clone()));
        let host_server = Arc::new(HostServer::new(&config.host_server, tool_index_sink));

        let tool_query_sink = Arc::new(RagQuerySink::new(tool_engine.clone(), host_server.clone()));
        let agent_server = Arc::new(AgentServer::new(&config.agent_server, tool_query_sink));

        Self {
            protocol_version: config.protocol_version,
            tool_engine,
            host_server,
            agent_server,
        }
    }
}
