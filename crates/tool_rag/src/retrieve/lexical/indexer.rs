/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod inverted;

use crate::model::{IndexedTool, ScoredHit, ToolDocField, ToolDocId};
use crate::retrieve::lexical::{LexicalRetrieveConfig, LexicalTokenizer};
use inverted::InvertedTable;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// 词法通道：倒排 + BM25F。`docs[i]` 的编号即 `ToolDocId`。
pub(crate) struct LexicalIndexer {
    inverted_table: InvertedTable,

    /// dl(d)：文档 d 的加权长度 = Σ (字段词元数 × 字段权重)
    doc_weights: Vec<f32>,

    /// avg_dl：dl 的均值；空库或全 0 时为 1
    avg_dl: f32,

    /// N：文档总数
    count: usize,

    config: LexicalRetrieveConfig,
}

impl Default for LexicalIndexer {
    fn default() -> Self {
        Self {
            inverted_table: InvertedTable::default(),
            doc_weights: Vec::default(),
            avg_dl: 1.0,
            count: 0,
            config: LexicalRetrieveConfig::default(),
        }
    }
}

impl LexicalIndexer {
    pub fn build(docs: &[Arc<IndexedTool>], config: &LexicalRetrieveConfig) -> Self {
        let count = docs.len();
        let mut doc_weights: Vec<f32> = vec![0.0; count];
        let mut inverted_table = InvertedTable::new();

        for (index, tool) in docs.iter().enumerate() {
            let fields = tool.search_fields();

            Self::index_field(
                &mut doc_weights,
                &mut inverted_table,
                index,
                ToolDocField::Name,
                fields.name,
                config.field_weight(ToolDocField::Name),
            );

            Self::index_field(
                &mut doc_weights,
                &mut inverted_table,
                index,
                ToolDocField::Description,
                fields.description,
                config.field_weight(ToolDocField::Description),
            );

            for tag in fields.tags {
                Self::index_field(
                    &mut doc_weights,
                    &mut inverted_table,
                    index,
                    ToolDocField::Tags,
                    tag,
                    config.field_weight(ToolDocField::Tags),
                );
            }
        }

        let sum: f32 = doc_weights.iter().sum();
        let avg_dl = if count == 0 || sum == 0.0 {
            1.0
        } else {
            sum / (count as f32)
        };

        Self {
            inverted_table,
            doc_weights,
            avg_dl,
            count,
            config: config.clone(),
        }
    }

    /// BM25F： score(d,q) = Σ_t IDF(t) × norm(tf'(t,d), dl(d))
    pub fn search(&self, query: &str) -> Vec<ScoredHit> {
        let tokens = LexicalTokenizer::tokenize(query);
        if tokens.is_empty() || self.is_empty() {
            return Vec::new();
        }

        // 每篇文档的得分
        let mut scores: HashMap<ToolDocId, f32> = HashMap::new();

        for t in &tokens {
            let tfs = self.tf_by_doc(t);
            if tfs.is_empty() {
                continue;
            }

            let idf = self.idf(t);
            for (doc_id, tf) in tfs {
                // score(d) += IDF(t) × norm(tf', dl)
                let contrib = idf * self.norm(self.doc_weights[doc_id], tf);
                *scores.entry(doc_id).or_insert(0.0) += contrib;
            }
        }

        let mut hits: Vec<ScoredHit> = scores
            .into_iter()
            .map(|(doc_id, score)| ScoredHit { doc_id, score })
            .collect();

        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        hits.truncate(self.config.candidate);

        hits
    }

    fn index_field(
        doc_weights: &mut [f32],
        inverted_table: &mut InvertedTable,
        index: usize,
        field: ToolDocField,
        text: &str,
        weight: f32,
    ) {
        let tokens = LexicalTokenizer::tokenize(text);

        if let Some(w) = doc_weights.get_mut(index) {
            let v = weight * (tokens.len() as f32);
            *w += v;
        }

        inverted_table.insert(index, field, tokens);
    }

    /// df(t)：term 出现在多少篇不同文档里
    fn df(&self, term: &str) -> usize {
        let mut set = HashSet::new();

        for p in self.inverted_table.get(term) {
            set.insert(p.doc_id);
        }

        set.len()
    }

    /// tf'(t,d) = Σ posting tf × w_field
    fn tf_by_doc(&self, term: &str) -> HashMap<ToolDocId, f32> {
        let mut tfs: HashMap<ToolDocId, f32> = HashMap::new();

        let postings = self.inverted_table.get(term);

        for p in postings {
            let tf = tfs.entry(p.doc_id).or_insert(0.0);
            *tf += (p.tf as f32) * self.config.field_weight(p.field);
        }

        tfs
    }

    /// IDF(t) = ln((N - df + 0.5) / (df + 0.5) + 1)，越大越稀有
    fn idf(&self, term: &str) -> f32 {
        let seen = self.df(term);
        if seen == 0 {
            return 0.0;
        }

        let n = self.count as f32;
        let df = seen as f32;

        let idf = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();

        idf
    }

    /// norm = tf'×(k1+1) / (tf' + k1×(1 - b + b×dl/avgdl))
    fn norm(&self, dl: f32, tf: f32) -> f32 {
        let k1 = self.config.k1;
        let b = self.config.b;
        let avg_dl = self.avg_dl;

        let up = tf * (k1 + 1.0);
        let down = tf + k1 * (1.0 - b + b * (dl / avg_dl));

        up / down
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bento_protocol::tool::ToolDefinition;

    fn tool(name: &str, description: &str, tags: &[&str]) -> Arc<IndexedTool> {
        Arc::new(IndexedTool::new(
            "blender",
            ToolDefinition {
                name: name.into(),
                description: description.into(),
                input_schema: serde_json::json!({ "type": "object" }),
                risk: Default::default(),
                tags: tags.iter().map(|s| s.to_string()).collect(),
            },
        ))
    }

    fn index_with(docs: Vec<Arc<IndexedTool>>, candidate: usize) -> LexicalIndexer {
        let mut config = LexicalRetrieveConfig::default();
        config.candidate = candidate;
        LexicalIndexer::build(&docs, &config)
    }

    fn index(docs: Vec<Arc<IndexedTool>>) -> LexicalIndexer {
        index_with(docs, 5)
    }

    fn docs() -> Vec<Arc<IndexedTool>> {
        vec![
            tool(
                r#"createCube"#,
                r#"crate a cube."#,
                &[r#"modeling"#, r#"cube"#],
            ),
            tool(r#"edit"#, r#"编辑立方体"#, &[r#"modeling"#, r#"cube"#]),
            tool(r#"export_1"#, r#"export a cube"#, &[r#"export"#, r#"cube"#]),
            tool(r#"export_2"#, r#"export a fbx"#, &[r#"assert"#]),
            tool(r#"tool"#, r#"export a assert"#, &[r#"utility"#]),
        ]
    }

    #[test]
    fn test_search() {
        let indexer = index_with(docs(), 3);
        println!("{:?}", indexer.search("cube"));
        println!("{:?}", indexer.search("方体"));
        println!("{:?}", indexer.search("export assert"));
    }
}
