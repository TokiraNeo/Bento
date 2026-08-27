/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::ToolRagConfig;
use crate::model::IndexedTool;
use crate::retrieve::exact::ExactIndexer;
use crate::retrieve::lexical::LexicalIndexer;
use crate::retrieve::semantic::SemanticIndexer;
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
}
