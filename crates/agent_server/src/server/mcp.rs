/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
mod tool_params;

use crate::ToolQuerySink;
use bento_protocol::jsonrpc::results::ToolCallResult;
use bento_protocol::jsonrpc::templates::from_response;
use bento_protocol::tool::{ToolCallContent, ToolCallContentType, ToolSearchQuery};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ProtocolVersion, ServerCapabilities, ServerInfo};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
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
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                //.enable_prompts()
                .build(),
        )
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
    async fn search_tools(&self, Parameters(p): Parameters<SearchToolParams>) -> ToolCallResult {
        let query = ToolSearchQuery {
            text: p.text,
            top_k: p.top_k,
        };

        match self.sink.search_tools(query).await {
            Ok(hits) => ToolCallResult {
                content: vec![ToolCallContent {
                    content_type: ToolCallContentType::Text,
                    text: serde_json::to_string(&hits).unwrap_or_default(),
                }],
                is_error: false,
            },

            Err(err) => ToolCallResult {
                content: vec![ToolCallContent {
                    content_type: ToolCallContentType::Text,
                    text: err.to_string(),
                }],
                is_error: true,
            },
        }
    }

    #[tool(
        name = "bento.get_tool_schema",
        description = "取回工具完整 input_schema"
    )]
    async fn get_tool_schema(
        &self,
        Parameters(p): Parameters<GetToolSchemaParams>,
    ) -> ToolCallResult {
        match self.sink.get_tool_schema(&p.name).await {
            Ok(schema) => ToolCallResult {
                content: vec![ToolCallContent {
                    content_type: ToolCallContentType::Text,
                    text: serde_json::to_string(&schema).unwrap_or_default(),
                }],
                is_error: false,
            },

            Err(err) => ToolCallResult {
                content: vec![ToolCallContent {
                    content_type: ToolCallContentType::Text,
                    text: err.to_string(),
                }],
                is_error: true,
            },
        }
    }

    #[tool(name = "bento.call_tool", description = "调用具体的工具")]
    async fn call_tool(&self, Parameters(p): Parameters<CallToolParams>) -> ToolCallResult {
        let timeout = Duration::from_millis(p.timeout_ms);
        self.sink
            .call_tool(&p.name, p.arguments, timeout)
            .await
            .unwrap_or_else(|err| ToolCallResult {
                content: vec![ToolCallContent {
                    content_type: ToolCallContentType::Text,
                    text: err.to_string(),
                }],
                is_error: true,
            })
    }
}
