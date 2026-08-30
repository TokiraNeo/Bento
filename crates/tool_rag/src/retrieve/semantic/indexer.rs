/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::model::{IndexedTool, SearchHit};
use crate::{EmbedVector, SemanticRetrieveConfig};
use std::sync::{Arc, RwLock};

/// 语义通道检索器
pub(crate) struct SemanticIndexer {
    config: SemanticRetrieveConfig,

    /// doc_id -> EmbedVector
    embeddings: RwLock<Vec<Option<EmbedVector>>>,
}

impl Default for SemanticIndexer {
    fn default() -> Self {
        Self {
            config: SemanticRetrieveConfig::default(),
            embeddings: RwLock::new(Vec::new()),
        }
    }
}

impl SemanticIndexer {
    pub fn build(docs: &[Arc<IndexedTool>], config: &SemanticRetrieveConfig) -> Self {
        Self {
            config: config.clone(),
            embeddings: RwLock::new(vec![None; docs.len()]),
        }
    }

    pub fn search(&self, embedding: EmbedVector) -> Vec<SearchHit> {
        if embedding.is_empty() {
            return Vec::new();
        }

        todo!()
    }

    pub fn update(&self, embeddings: Vec<EmbedVector>) {
        let mut store = self.embeddings.write().unwrap();

        if (embeddings.is_empty() || embeddings.len() != store.len()) {
            return;
        }

        // old = store[i], new = embeddings[i]
        for (old, new) in store.iter_mut().zip(embeddings) {
            *old = Some(new);
        }
    }
}
