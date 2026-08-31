/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::FusionConfig;
use crate::model::{SearchHit, ToolDocId};
use std::collections::HashMap;

pub(crate) struct FusionScaler;

impl FusionScaler {
    pub fn rrf(
        config: &FusionConfig,
        exact: Vec<SearchHit>,
        lexical: Vec<SearchHit>,
        semantic: Vec<SearchHit>,
    ) -> Vec<SearchHit> {
        let mut rrf_scores: HashMap<ToolDocId, f32> = HashMap::new();

        for (w, hits) in [
            (config.exact, exact.as_slice()),
            (config.lexical, lexical.as_slice()),
            (config.semantic, semantic.as_slice()),
        ] {
            for (i, hit) in hits.iter().enumerate() {
                let d = (i + 1) as f32;
                let v = w / (config.rrf_k + d);

                let score = rrf_scores.entry(hit.doc_id).or_insert(0f32);
                *score += v;
            }
        }

        let mut results: Vec<SearchHit> = rrf_scores
            .into_iter()
            .map(|(doc_id, score)| SearchHit { doc_id, score })
            .collect();

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }
}
