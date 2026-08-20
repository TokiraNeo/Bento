/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::config::ToolRagConfig;
use bento_protocol::tool::{ToolDefinition, ToolSearchHit, ToolSearchQuery};
use serde_json::Value;
use std::borrow::Cow;

pub struct ToolRagEngine {
    config: ToolRagConfig,
}

impl ToolRagEngine {
    pub fn new(config: ToolRagConfig) -> Self {
        Self { config }
    }

    pub async fn replace_host_tools(
        &self,
        session_id: &str,
        namespace: &str,
        tools: Vec<ToolDefinition>,
    ) -> Result<usize, Cow<'static, str>> {
        todo!()
    }

    pub fn mark_host_ready(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        todo!()
    }

    pub fn remove_host_tools(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        todo!()
    }

    pub async fn search_tools(
        &self,
        query: ToolSearchQuery,
    ) -> Result<Vec<ToolSearchHit>, Cow<'static, str>> {
        todo!()
    }

    pub fn get_tool_schema(&self, qualified_name: &str) -> Result<Value, Cow<'static, str>> {
        todo!()
    }

    pub fn list_domains(&self) -> Result<Vec<String>, Cow<'static, str>> {
        todo!()
    }
}
