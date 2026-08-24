/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::catalog::{ToolBucket, ToolCatalog};
use crate::config::ToolRagConfig;
use crate::snapshot::SearchSnapshot;
use bento_protocol::tool::{ToolDefinition, ToolSearchHit, ToolSearchQuery};
use serde_json::Value;
use std::borrow::Cow;
use std::sync::{Arc, RwLock};

pub struct ToolRagEngine {
    config: ToolRagConfig,
    catalog: ToolCatalog,
    snapshot: RwLock<Arc<SearchSnapshot>>,
}

impl ToolRagEngine {
    pub fn new(config: ToolRagConfig) -> Self {
        Self {
            config,
            catalog: ToolCatalog::new(),
            snapshot: RwLock::default(),
        }
    }

    pub async fn replace_host_tools(
        &self,
        session_id: &str,
        namespace: &str,
        tools: Vec<ToolDefinition>,
    ) -> Result<usize, Cow<'static, str>> {
        let count = self
            .catalog
            .replace(session_id.to_owned(), ToolBucket::new(namespace, tools))?;
        self.rebuild_snapshot();
        Ok(count)
    }

    pub fn mark_host_ready(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        self.catalog.mark_ready(session_id)?;
        self.rebuild_snapshot();
        Ok(())
    }

    pub fn remove_host_tools(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        self.catalog.remove(session_id)?;
        self.rebuild_snapshot();
        Ok(())
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

    fn rebuild_snapshot(&self) {
        let docs = self.catalog.ready_tools();
        let snapshot = Arc::new(SearchSnapshot::build(docs, &self.config));
        *self.snapshot.write().unwrap() = snapshot;
    }
}
