/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
mod tool_params;

use crate::ToolQuerySink;
use bento_protocol::tool::{ToolRisk, ToolSearchQuery};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, ContentBlock, ProtocolVersion, ServerCapabilities, ServerInfo};
use rmcp::{ErrorData as McpError, ServerHandler, tool, tool_handler, tool_router};
use std::sync::Arc;
use std::time::Duration;
use tool_params::*;

#[derive(Clone)]
pub(super) struct AgentMcpServer {
    sink: Arc<dyn ToolQuerySink>,
}

#[tool_handler]
impl ServerHandler for AgentMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2026_07_28)
            .with_instructions(
                "Bento工具搜索引擎：先 search_tools 搜索，再 get_tool_schema 取 schema，最后 call_tool 调用"
                    .to_string(),
            )
    }
}

#[tool_router]
impl AgentMcpServer {
    pub fn new(sink: Arc<dyn ToolQuerySink>) -> Self {
        Self { sink }
    }

    #[tool(
        name = "bento.search_tools",
        description = "搜索工具：全名精确命中优先，其次关键词/语义"
    )]
    async fn search_tools(
        &self,
        Parameters(p): Parameters<SearchToolParams>,
    ) -> Result<CallToolResult, McpError> {
        let query = ToolSearchQuery {
            text: p.text,
            top_k: p.top_k,
        };

        match self.sink.search_tools(query).await {
            Ok(hits) => Ok(CallToolResult::success(vec![ContentBlock::text(
                serde_json::to_string(&hits).unwrap_or_default(),
            )])),

            Err(err) => Ok(CallToolResult::error(vec![ContentBlock::text(
                err.to_string(),
            )])),
        }
    }

    #[tool(
        name = "bento.get_tool_schema",
        description = "取回工具完整 input_schema"
    )]
    async fn get_tool_schema(
        &self,
        Parameters(p): Parameters<GetToolSchemaParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.sink.get_tool_schema(&p.name).await {
            Ok(schema) => Ok(CallToolResult::success(vec![ContentBlock::text(
                serde_json::to_string(&schema).unwrap_or_default(),
            )])),

            Err(err) => Ok(CallToolResult::error(vec![ContentBlock::text(
                err.to_string(),
            )])),
        }
    }

    #[tool(name = "bento.call_tool", description = "调用具体的工具")]
    async fn call_tool(
        &self,
        Parameters(p): Parameters<CallToolParams>,
    ) -> Result<CallToolResult, McpError> {
        let timeout = Duration::from_millis(p.timeout_ms);

        match self.sink.call_tool(&p.name, p.arguments, timeout).await {
            Ok(result) => {
                let content: Vec<ContentBlock> = result
                    .content
                    .into_iter()
                    .map(|c| ContentBlock::text(c.text))
                    .collect();

                if result.is_error {
                    Ok(CallToolResult::error(content))
                } else {
                    Ok(CallToolResult::success(content))
                }
            }

            Err(err) => Ok(CallToolResult::error(vec![ContentBlock::text(
                err.to_string(),
            )])),
        }
    }
}
