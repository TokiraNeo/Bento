/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::catalog::{ToolBucket, ToolCatalog};
use crate::config::ToolRagConfig;
use crate::snapshot::SearchSnapshot;
use crate::{EmbedVector, Embedder};
use bento_protocol::tool::{
    ToolDefinition, ToolRisk, ToolSchema, ToolSearchQuery, ToolSearchResult,
};
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
    snapshot_sender: mpsc::Sender<()>, // 通知重建快照
    embedder: Option<Arc<dyn Embedder>>,
    embed_sender: mpsc::Sender<Arc<SearchSnapshot>>, // 重建语义向量库
}

impl ToolRagEngine {
    pub fn new(config: &ToolRagConfig, embedder: Option<Arc<dyn Embedder>>) -> Arc<Self> {
        let (snapshot_sender, snapshot_receiver) = mpsc::channel();
        let (embed_sender, embed_receiver) = mpsc::channel::<Arc<SearchSnapshot>>();

        let engine = Arc::new(Self {
            version: AtomicUsize::new(0),
            config: config.clone(),
            catalog: ToolCatalog::new(),
            snapshot: RwLock::default(),
            snapshot_sender,
            embedder,
            embed_sender,
        });

        let weak = Arc::downgrade(&engine);

        let weak_cloned = weak.clone();
        std::thread::spawn(move || Self::merge_snapshot(weak_cloned, snapshot_receiver));

        std::thread::spawn(move || Self::merge_embedding(weak, embed_receiver));

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
        self.mark_dirty()?;
        Ok(count)
    }

    pub async fn mark_host_ready(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        self.catalog.mark_ready(session_id)?;
        self.mark_dirty()
    }

    pub async fn remove_host_tools(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        self.catalog.remove(session_id)?;
        self.mark_dirty()
    }

    pub async fn search_tools(
        &self,
        query: ToolSearchQuery,
    ) -> Result<Vec<ToolSearchResult>, Cow<'static, str>> {
        let snapshot = self.snapshot.read().unwrap().clone();

        let mut embedding: EmbedVector = Vec::new();

        if let Some(embedder) = &self.embedder {
            match embedder.embed_query(&query.text) {
                Ok(v) => {
                    embedding = v;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        let results = snapshot
            .search_tools(&query.text, query.top_k, embedding, &self.config.fusion)
            .await;

        Ok(results)
    }

    pub fn get_tool_schema(&self, qualified_name: &str) -> Result<ToolSchema, Cow<'static, str>> {
        self.snapshot
            .read()
            .unwrap()
            .get_tool_schema(qualified_name)
    }

    pub fn get_tool_risk(&self, qualified_name: &str) -> Result<ToolRisk, Cow<'static, str>> {
        self.snapshot.read().unwrap().get_tool_risk(qualified_name)
    }

    fn mark_dirty(&self) -> Result<(), Cow<'static, str>> {
        self.version.fetch_add(1, Acquire);

        match self.snapshot_sender.send(()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Cow::Owned(err.to_string())),
        }
    }

    fn merge_snapshot(weak_self: Weak<Self>, mut receiver: mpsc::Receiver<()>) {
        while receiver.recv().is_ok() {
            // 将当前积压的触发全部处理
            while receiver.try_recv().is_ok() {}

            let Some(engine) = weak_self.upgrade() else {
                break;
            };

            let version = engine.version.load(Acquire);
            let docs = engine.catalog.ready_tools();
            let snapshot = Arc::new(SearchSnapshot::build(version, docs, &engine.config));

            if snapshot.version != engine.version.load(Acquire) {
                continue;
            }

            *engine.snapshot.write().unwrap() = snapshot.clone();

            // 异步更新向量
            let _ = engine.embed_sender.send(snapshot);
        }
    }

    fn merge_embedding(weak_self: Weak<Self>, mut receiver: mpsc::Receiver<Arc<SearchSnapshot>>) {
        while let Ok(job) = receiver.recv() {
            {
                let mut latest = job;

                // 处理所有积压，保留最新一份
                while let Ok(update) = receiver.try_recv() {
                    latest = update;
                }

                let Some(engine) = weak_self.upgrade() else {
                    break;
                };

                let Some(embedder) = &engine.embedder else {
                    break;
                };

                let docs = latest.semantic_docs();

                if let Ok(embeddings) = embedder.embed_docs(&docs) {
                    if latest.version != engine.version.load(Acquire) {
                        continue;
                    }

                    latest.update_embeddings(embeddings);
                }
            }
        }
    }
}
