/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::ExactRetrieveConfig;
use crate::model::{IndexedTool, SearchHit, ToolDocId};
use std::collections::HashMap;
use std::sync::Arc;

/// 精确通道的检索器：直接通过qualified_name检索，例如“blender.create_cube”
pub(crate) struct ExactIndexer {
    config: ExactRetrieveConfig,

    /// qualified_name - doc_id
    map: HashMap<String, Vec<ToolDocId>>,
}

impl Default for ExactIndexer {
    fn default() -> Self {
        Self {
            config: ExactRetrieveConfig::default(),
            map: HashMap::new(),
        }
    }
}

impl ExactIndexer {
    pub fn build(docs: &[Arc<IndexedTool>], config: &ExactRetrieveConfig) -> Self {
        let mut map: HashMap<String, Vec<ToolDocId>> = HashMap::new();

        for (index, tool) in docs.iter().enumerate() {
            // 这里用裸的host名拼接，这样能让例如"blender.export"、"blender#2.export"都能被"blender.export"检索命中
            let term = format!("{}.{}", tool.host_name, tool.definition.name);

            map.entry(term).or_default().push(index);
        }

        Self {
            config: config.clone(),
            map,
        }
    }

    pub fn search(&self, query: &str) -> Vec<SearchHit> {
        if query.is_empty() {
            return Vec::new();
        }

        let mut hits: Vec<SearchHit> = Vec::new();

        if let Some(ids) = self.map.get(query) {
            for i in ids {
                hits.push(SearchHit {
                    doc_id: *i,
                    score: 1.0,
                });
            }
        }

        hits.truncate(self.config.candidate);

        hits
    }
}
