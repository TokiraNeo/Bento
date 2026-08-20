/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use async_trait::async_trait;
use bento_agent_server::ToolQuerySink;
use bento_protocol::tool::{ToolSearchHit, ToolSearchQuery};
use bento_tool_rag::ToolRagEngine;
use serde_json::Value;
use std::borrow::Cow;
use std::sync::Arc;

pub struct RagQuerySink(Arc<ToolRagEngine>);

impl RagQuerySink {
    pub fn new(engine: Arc<ToolRagEngine>) -> Self {
        Self(engine)
    }
}

#[async_trait]
impl ToolQuerySink for RagQuerySink {
    async fn search_tools(
        &self,
        query: ToolSearchQuery,
    ) -> Result<Vec<ToolSearchHit>, Cow<'static, str>> {
        self.0.search_tools(query).await
    }

    async fn get_tool_schema(&self, qualified_name: &str) -> Result<Value, Cow<'static, str>> {
        self.0.get_tool_schema(qualified_name)
    }

    async fn list_domains(&self) -> Result<Vec<String>, Cow<'static, str>> {
        self.0.list_domains()
    }
}
