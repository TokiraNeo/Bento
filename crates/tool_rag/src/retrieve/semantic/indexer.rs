/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::SemanticRetrieveConfig;
use crate::model::IndexedTool;
use std::sync::Arc;

/// 语义通道检索器
pub(crate) struct SemanticIndexer {
    config: SemanticRetrieveConfig,
}

impl Default for SemanticIndexer {
    fn default() -> Self {
        Self {
            config: SemanticRetrieveConfig::default(),
        }
    }
}

impl SemanticIndexer {
    pub fn build(docs: &[Arc<IndexedTool>], config: &SemanticRetrieveConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}
