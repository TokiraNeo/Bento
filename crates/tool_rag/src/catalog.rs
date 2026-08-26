/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::model::IndexedTool;
use bento_protocol::tool::ToolDefinition;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub(crate) struct ToolBucket {
    pub namespace: String,
    ready: bool,
    pub tools: Vec<Arc<IndexedTool>>,
}

impl ToolBucket {
    pub fn new(namespace: &str, definitions: Vec<ToolDefinition>) -> Self {
        let tools = definitions
            .into_iter()
            .map(|definition| Arc::new(IndexedTool::new(namespace, definition)))
            .collect();

        Self {
            namespace: namespace.to_owned(),
            ready: false,
            tools,
        }
    }

    pub fn mark_ready(&mut self) {
        self.ready = true;
    }
}

pub(crate) struct ToolCatalog {
    /// session_id - ToolBucket
    buckets: RwLock<HashMap<String, ToolBucket>>,
}

impl ToolCatalog {
    pub fn new() -> Self {
        Self {
            buckets: RwLock::new(HashMap::new()),
        }
    }

    pub fn replace(
        &self,
        session_id: String,
        bucket: ToolBucket,
    ) -> Result<usize, Cow<'static, str>> {
        let mut buckets = self.buckets.write().unwrap();

        let len = bucket.tools.len();

        buckets.remove(&session_id);

        buckets.insert(session_id, bucket);

        Ok(len)
    }

    pub fn mark_ready(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        let mut buckets = self.buckets.write().unwrap();

        if !buckets.contains_key(session_id) {
            return Err(Cow::Borrowed("No such session"));
        }

        let bucket = buckets.get_mut(session_id).unwrap();
        bucket.mark_ready();

        Ok(())
    }

    pub fn remove(&self, session_id: &str) -> Result<(), Cow<'static, str>> {
        let mut buckets = self.buckets.write().unwrap();

        if !buckets.contains_key(session_id) {
            return Err(Cow::Borrowed("No such session"));
        }

        buckets.remove(session_id);

        Ok(())
    }

    pub fn ready_tools(&self) -> Vec<Arc<IndexedTool>> {
        let buckets = self.buckets.read().unwrap();
        buckets
            .values()
            .filter(|b| b.ready)
            .flat_map(|b| b.tools.iter().cloned())
            .collect()
    }
}
