/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::fusion::FusionScaler;
use crate::model::IndexedTool;
use crate::retrieve::exact::ExactIndexer;
use crate::retrieve::lexical::LexicalIndexer;
use crate::retrieve::semantic::SemanticIndexer;
use crate::{FusionConfig, ToolRagConfig};
use bento_protocol::tool::{ToolSearchQuery, ToolSearchResult};
use serde_json::Value;
use std::borrow::Cow;
use std::sync::Arc;

pub(crate) struct SearchSnapshot {
    pub version: usize,
    docs: Vec<Arc<IndexedTool>>,
    exact_indexer: ExactIndexer,
    lexical_indexer: LexicalIndexer,
    semantic_indexer: SemanticIndexer,
}

impl Default for SearchSnapshot {
    fn default() -> Self {
        Self {
            version: 0,
            docs: Vec::new(),
            exact_indexer: ExactIndexer::default(),
            lexical_indexer: LexicalIndexer::default(),
            semantic_indexer: SemanticIndexer::default(),
        }
    }
}

impl SearchSnapshot {
    pub fn build(version: usize, docs: Vec<Arc<IndexedTool>>, config: &ToolRagConfig) -> Self {
        std::thread::scope(|s| {
            let exact = s.spawn(|| ExactIndexer::build(&docs, &config.exact));
            let lexical = s.spawn(|| LexicalIndexer::build(&docs, &config.lexical));
            let semantic = s.spawn(|| SemanticIndexer::build(&docs, &config.semantic));

            Self {
                version,
                docs,
                exact_indexer: exact.join().unwrap(),
                lexical_indexer: lexical.join().unwrap(),
                semantic_indexer: semantic.join().unwrap(),
            }
        })
    }

    pub async fn search_tools(
        &self,
        query: ToolSearchQuery,
        config: &FusionConfig,
    ) -> Vec<ToolSearchResult> {
        std::thread::scope(|s| {
            let exact = s.spawn(|| self.exact_indexer.search(&query.text));
            let lexical = s.spawn(|| self.lexical_indexer.search(&query.text));
            let semantic = s.spawn(|| self.semantic_indexer.search(&query.text));

            let mut hits = FusionScaler::rrf(
                config,
                exact.join().unwrap(),
                lexical.join().unwrap(),
                semantic.join().unwrap(),
            );

            hits.truncate(query.top_k);

            let mut results: Vec<ToolSearchResult> = Vec::new();

            for hit in &hits {
                if let Some(tool) = self.docs.get(hit.doc_id) {
                    let def = &tool.definition;

                    results.push(ToolSearchResult {
                        qualified_name: format!("{}.{}", tool.namespace, def.name),
                        description: def.description.clone(),
                    });
                }
            }

            results
        })
    }

    pub fn get_tool_schema(&self, qualified_name: &str) -> Result<Value, Cow<'static, str>> {
        let found = self
            .docs
            .iter()
            .find(|tool| format!("{}.{}", tool.namespace, tool.definition.name) == qualified_name)
            .cloned();

        match found {
            Some(tool) => match serde_json::to_value(&tool.definition.input_schema) {
                Ok(value) => Ok(value),
                Err(err) => Err(Cow::Owned(err.to_string())),
            },

            None => Err(Cow::Borrowed("Tool not found")),
        }
    }
}
