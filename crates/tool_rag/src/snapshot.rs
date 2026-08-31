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
use crate::{EmbedVector, FusionConfig, ToolRagConfig};
use bento_protocol::tool::{ToolSchema, ToolSearchResult};
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
            let exact = {
                let docs = docs.clone();
                s.spawn(move || ExactIndexer::build(&docs, &config.exact))
            };

            let lexical = {
                let docs = docs.clone();
                s.spawn(move || LexicalIndexer::build(&docs, &config.lexical))
            };
            
            let semantic = {
                let docs = docs.clone();
                s.spawn(move || SemanticIndexer::build(&docs, &config.semantic))
            };

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
        query: &str,
        top_k: usize,
        embedding: EmbedVector,
        config: &FusionConfig,
    ) -> Vec<ToolSearchResult> {
        std::thread::scope(|s| {
            let exact = s.spawn(|| self.exact_indexer.search(query));
            let lexical = s.spawn(|| self.lexical_indexer.search(query));
            let semantic = s.spawn(|| self.semantic_indexer.search(embedding));

            let mut hits = FusionScaler::rrf(
                config,
                exact.join().unwrap(),
                lexical.join().unwrap(),
                semantic.join().unwrap(),
            );

            hits.truncate(top_k);

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

    pub fn get_tool_schema(&self, qualified_name: &str) -> Result<ToolSchema, Cow<'static, str>> {
        let found = self
            .docs
            .iter()
            .find(|tool| format!("{}.{}", tool.namespace, tool.definition.name) == qualified_name)
            .cloned();

        match found {
            Some(tool) => match serde_json::to_value(&tool.definition.input_schema) {
                Ok(value) => Ok(ToolSchema {
                    name: qualified_name.to_string(),
                    input_schema: value,
                }),
                Err(err) => Err(Cow::Owned(err.to_string())),
            },

            None => Err(Cow::Borrowed("Tool not found")),
        }
    }

    pub fn semantic_docs(&self) -> Vec<String> {
        self.docs
            .iter()
            .map(|tool| tool.semantic_query.clone())
            .collect()
    }

    pub fn update_embeddings(&self, embeddings: Vec<EmbedVector>) {
        self.semantic_indexer.update(embeddings);
    }
}
