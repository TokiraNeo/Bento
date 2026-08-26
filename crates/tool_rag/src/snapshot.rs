/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::model::IndexedTool;
use crate::retrieve::lexical::LexicalIndexer;
use crate::retrieve::lexical::LexicalRetrieveConfig;
use std::sync::Arc;

pub(crate) struct SearchSnapshot {
    docs: Vec<Arc<IndexedTool>>,
    lexical_indexer: LexicalIndexer,
}

impl Default for SearchSnapshot {
    fn default() -> Self {
        Self {
            docs: Vec::new(),
            lexical_indexer: LexicalIndexer::default(),
        }
    }
}

impl SearchSnapshot {
    pub fn build(docs: Vec<Arc<IndexedTool>>, lexical: &LexicalRetrieveConfig) -> Self {
        let indexer = LexicalIndexer::build(&docs, lexical);
        Self {
            docs,
            lexical_indexer: indexer,
        }
    }
}
