/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::catalog::{ToolBucket, ToolCatalog};
use crate::config::ToolRagConfig;
use crate::snapshot::SearchSnapshot;
use bento_protocol::tool::{ToolDefinition, ToolSearchQuery, ToolSearchResult};
use serde_json::Value;
use std::borrow::Cow;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Acquire;
use std::sync::mpsc;
use std::sync::{Arc, RwLock, Weak};

pub struct ToolRagEngine {
    version: AtomicUsize,
    config: ToolRagConfig,
    catalog: ToolCatalog,
    snapshot: RwLock<Arc<SearchSnapshot>>,
    rebuild_sender: mpsc::Sender<()>,
}

impl ToolRagEngine {
    pub fn new(config: &ToolRagConfig) -> Arc<Self> {
        let (sender, mut receiver) = mpsc::channel();

        let engine = Arc::new(Self {
            version: AtomicUsize::new(0),
            config: config.clone(),
            catalog: ToolCatalog::new(),
            snapshot: RwLock::default(),
            rebuild_sender: sender,
        });

        let weak = Arc::downgrade(&engine);

        std::thread::spawn(move || Self::merge_snapshot(weak, receiver));

        engine
    }

    pub async fn replace_host_tools(
        &self,
        session_id: &str,
        name: &str,
        namespace: &str,
        tools: Vec<ToolDefinition>,
    ) -> Result<usize, Cow<'static, str>> {
        let count = self.catalog.replace(
            session_id.to_owned(),
            ToolBucket::new(name, namespace, tools),
        )?;
        self.mark_dirty().await?;
        Ok(count)
    }

    pub async fn mark_host_ready(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        self.catalog.mark_ready(session_id)?;
        self.mark_dirty().await
    }

    pub async fn remove_host_tools(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        self.catalog.remove(session_id)?;
        self.mark_dirty().await
    }

    pub async fn search_tools(
        &self,
        query: ToolSearchQuery,
    ) -> Result<Vec<ToolSearchResult>, Cow<'static, str>> {
        todo!()
    }

    pub fn get_tool_schema(&self, qualified_name: &str) -> Result<Value, Cow<'static, str>> {
        todo!()
    }

    async fn mark_dirty(&self) -> Result<(), Cow<'static, str>> {
        self.version.fetch_add(1, Acquire);

        match self.rebuild_sender.send(()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Cow::Owned(err.to_string())),
        }
    }

    async fn merge_snapshot(weak_self: Weak<Self>, mut receiver: mpsc::Receiver<()>) {
        while receiver.recv().is_ok() {
            // 将当前积压的触发全部处理
            while receiver.try_recv().is_ok() {}

            let Some(engine) = weak_self.upgrade() else {
                break;
            };

            let version = engine.version.load(Acquire);
            let docs = engine.catalog.ready_tools();
            let snapshot = SearchSnapshot::build(version, docs, &engine.config);

            *engine.snapshot.write().unwrap() = Arc::new(snapshot);
        }
    }
}
