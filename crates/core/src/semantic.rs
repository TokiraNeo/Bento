/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use bento_tool_rag::{EmbedVector, Embedder};
use bento_utility::maths::normalize;
use fastembed::{EmbeddingModel, TextEmbedding, TextInitOptions};
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

pub(crate) struct SemanticEmbedder {
    model: Arc<Mutex<TextEmbedding>>,
}

impl SemanticEmbedder {
    pub fn new() -> Result<Self, Cow<'static, str>> {
        let model = TextEmbedding::try_new(
            TextInitOptions::new(EmbeddingModel::BGESmallZHV15).with_show_download_progress(false),
        )
        .map_err(|e| Cow::Owned(e.to_string()))?;

        Ok(Self {
            model: Arc::new(Mutex::new(model)),
        })
    }
}

impl Embedder for SemanticEmbedder {
    fn embed_docs(&self, docs: &[String]) -> Result<Vec<EmbedVector>, Cow<'static, str>> {
        if docs.is_empty() {
            return Ok(vec![]);
        }

        let mut model = self.model.lock().unwrap();

        let embeddings = model
            .embed(docs, None)
            .map_err(|e| Cow::Owned(e.to_string()))?;

        let normalized = embeddings
            .into_iter()
            .map(|vector| normalize(vector))
            .collect();

        Ok(normalized)
    }

    fn embed_query(&self, query: &str) -> Result<EmbedVector, Cow<'static, str>> {
        if query.is_empty() {
            return Ok(vec![]);
        }

        let mut model = self.model.lock().unwrap();

        let mut embedding = model
            .embed(&[query], None)
            .map_err(|e| Cow::Owned(e.to_string()))?;

        match embedding.pop() {
            None => Err(Cow::Borrowed("empty embedding")),

            Some(vector) => Ok(normalize(vector)),
        }
    }
}
