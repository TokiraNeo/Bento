/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use async_trait::async_trait;
use bento_agent_server::ToolQuerySink;
use bento_host_server::HostServer;
use bento_protocol::jsonrpc::params::ToolCallParam;
use bento_protocol::jsonrpc::results::ToolCallResult;
use bento_protocol::tool::{ToolSchema, ToolSearchQuery, ToolSearchResult};
use bento_tool_rag::ToolRagEngine;
use serde_json::Value;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

pub struct RagQuerySink {
    tool_engine: Arc<ToolRagEngine>,
    host_server: Arc<HostServer>,
}

impl RagQuerySink {
    pub fn new(tool_engine: Arc<ToolRagEngine>, host_server: Arc<HostServer>) -> Self {
        Self {
            tool_engine,
            host_server,
        }
    }
}

#[async_trait]
impl ToolQuerySink for RagQuerySink {
    async fn search_tools(
        &self,
        query: ToolSearchQuery,
    ) -> Result<Vec<ToolSearchResult>, Cow<'static, str>> {
        self.tool_engine.search_tools(query).await
    }

    async fn get_tool_schema(&self, qualified_name: &str) -> Result<ToolSchema, Cow<'static, str>> {
        self.tool_engine.get_tool_schema(qualified_name)
    }

    async fn call_tool(
        &self,
        qualified_name: &str,
        args: Value,
        timeout: Duration,
    ) -> Result<ToolCallResult, Cow<'static, str>> {
        let (namespace, tool_name) = qualified_name
            .split_once('.')
            .ok_or(Cow::Borrowed("Invalid qualified name"))?;

        let param = ToolCallParam {
            tool_name: tool_name.to_owned(),
            arguments: args,
        };

        self.host_server
            .call_tool(namespace.to_owned(), param, timeout)
            .await
    }
}
