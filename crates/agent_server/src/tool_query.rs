/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use async_trait::async_trait;
use bento_protocol::jsonrpc::results::ToolCallResult;
use bento_protocol::tool::{ToolRisk, ToolSchema, ToolSearchQuery, ToolSearchResult};
use serde_json::Value;
use std::borrow::Cow;
use std::time::Duration;

#[async_trait]
pub trait ToolQuerySink: Send + Sync {
    /// `bento.search_tools`：混合检索，返回工具列表。
    async fn search_tools(
        &self,
        query: ToolSearchQuery,
    ) -> Result<Vec<ToolSearchResult>, Cow<'static, str>>;

    /// `bento.get_tool_schema`：取回完整 input_schema。
    async fn get_tool_schema(&self, qualified_name: &str) -> Result<ToolSchema, Cow<'static, str>>;

    /// bento.call_tool: 调用工具
    async fn call_tool(
        &self,
        qualified_name: &str,
        args: Value,
        timeout: Duration,
    ) -> Result<ToolCallResult, Cow<'static, str>>;
}
