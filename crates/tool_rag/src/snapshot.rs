/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::ToolRagConfig;
use crate::lexical::LexicalIndexer;
use crate::model::IndexedTool;
use std::sync::Arc;

pub(crate) struct SearchSnapshot {
    docs: Vec<Arc<IndexedTool>>,
    indexer: LexicalIndexer,
}

impl Default for SearchSnapshot {
    fn default() -> Self {
        Self {
            docs: Vec::new(),
            indexer: LexicalIndexer::default(),
        }
    }
}

impl SearchSnapshot {
    pub fn build(docs: Vec<Arc<IndexedTool>>, config: &ToolRagConfig) -> Self {
        let indexer = LexicalIndexer::build(&docs, config);
        Self { docs, indexer }
    }
}
