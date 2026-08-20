/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::model::IndexedTool;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub(super) struct ToolBucket {
    pub namespace: String,
    ready: bool,
    pub tools: Vec<Arc<IndexedTool>>,
}

impl ToolBucket {
    pub fn new(namespace: String, tools: Vec<Arc<IndexedTool>>) -> Self {
        Self {
            namespace,
            ready: false,
            tools,
        }
    }

    pub fn mark_ready(&mut self) {
        self.ready = true;
    }
}

pub(super) struct ToolCatalog {
    /// session_id - ToolBucket
    buckets: RwLock<HashMap<String, ToolBucket>>,
}

impl ToolCatalog {
    pub fn new() -> Self {
        Self {
            buckets: RwLock::new(HashMap::new()),
        }
    }

    pub fn replace(&self, session_id: String, bucket: ToolBucket) -> Result<(), Cow<'static, str>> {
        let mut buckets = self.buckets.write().unwrap();

        buckets.remove(&session_id);

        buckets.insert(session_id, bucket);

        Ok(())
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
}
