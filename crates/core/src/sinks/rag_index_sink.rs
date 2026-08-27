/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use async_trait::async_trait;
use bento_host_server::ToolIndexSink;
use bento_protocol::tool::ToolDefinition;
use bento_tool_rag::ToolRagEngine;
use std::borrow::Cow;
use std::sync::Arc;

pub struct RagIndexSink(Arc<ToolRagEngine>);

impl RagIndexSink {
    pub fn new(engine: Arc<ToolRagEngine>) -> Self {
        Self(engine)
    }
}

#[async_trait]
impl ToolIndexSink for RagIndexSink {
    async fn replace(
        &self,
        session_id: &str,
        name: &str,
        namespace: &str,
        tools: Vec<ToolDefinition>,
    ) -> Result<usize, Cow<'static, str>> {
        self.0
            .replace_host_tools(session_id, name, namespace, tools)
            .await
    }

    async fn ready(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        self.0.mark_host_ready(session_id).await
    }

    async fn remove(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        self.0.remove_host_tools(session_id).await
    }
}
