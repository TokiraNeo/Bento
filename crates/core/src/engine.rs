/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::config::CoreConfig;
use crate::event::CoreEventBus;
use crate::sinks::{RagIndexSink, RagQuerySink};
use crate::{CoreEvent, ToolApprovalHandler};
use bento_agent_server::AgentServer;
use bento_host_server::HostServer;
use bento_tool_rag::ToolRagEngine;
use std::borrow::Cow;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct CoreEngine {
    protocol_version: String,

    bus: Arc<CoreEventBus>,

    tool_engine: Arc<ToolRagEngine>,
    host_server: Arc<HostServer>,
    agent_server: Arc<AgentServer>,
}

impl CoreEngine {
    pub fn new(config: CoreConfig, approval_handler: Arc<dyn ToolApprovalHandler>) -> Self {
        let tool_engine = ToolRagEngine::new(&config.tool_rag, None);

        let tool_index_sink = Arc::new(RagIndexSink::new(tool_engine.clone()));
        let host_server = Arc::new(HostServer::new(&config.host_server, tool_index_sink));

        let tool_query_sink = Arc::new(RagQuerySink::new(
            tool_engine.clone(),
            host_server.clone(),
            approval_handler,
        ));
        let agent_server = Arc::new(AgentServer::new(&config.agent_server, tool_query_sink));

        Self {
            protocol_version: config.protocol_version,
            bus: Arc::new(CoreEventBus::new()),
            tool_engine,
            host_server,
            agent_server,
        }
    }

    pub async fn run(&self) -> Result<(), Cow<'static, str>> {
        let clone_bus = self.bus.clone();
        let clone_host_server = self.host_server.clone();

        tokio::try_join!(
            self.host_server.run(),
            self.agent_server.run(),
            forward_host_events(clone_host_server, clone_bus)
        )?;

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<CoreEvent> {
        self.bus.subscribe()
    }

    pub fn stop(&self) {
        self.host_server.stop();
        self.agent_server.stop();
    }
}

impl Drop for CoreEngine {
    fn drop(&mut self) {
        self.stop();
    }
}

async fn forward_host_events(host_server: Arc<HostServer>, bus: Arc<CoreEventBus>) {
    let mut receiver = host_server.subscribe();

    loop {
        match receiver.recv().await {
            Err(_err) => break,

            Ok(_event) => {
                let hosts = host_server.list_hosts();
                let _ = bus.emit(CoreEvent::HostsChanged { hosts });
            }
        }
    }
}
