/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::model::{ToolDocField, ToolDocId};
use std::collections::{HashMap, HashSet};

pub(super) struct Posting {
    pub doc_id: ToolDocId,
    pub field: ToolDocField,

    /// term frequency: 词出现的次数
    pub tf: usize,
}

impl Posting {
    pub fn new(doc_id: ToolDocId, field: ToolDocField) -> Self {
        Self {
            doc_id,
            field,
            tf: 1,
        }
    }
}

/// 倒排表
pub(super) struct InvertedTable {
    map: HashMap<String, Vec<Posting>>,
}

impl InvertedTable {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, doc_id: ToolDocId, field: ToolDocField, terms: Vec<String>) {
        for term in terms {
            let postings = self.map.entry(term.clone()).or_default();

            match postings
                .iter_mut()
                .find(|p| p.doc_id == doc_id && p.field == field)
            {
                Some(p) => p.tf += 1,

                None => postings.push(Posting::new(doc_id, field)),
            }
        }
    }

    pub fn get(&self, term: &str) -> &[Posting] {
        self.map.get(term).map(Vec::as_slice).unwrap_or(&[])
    }

    /// document frequency: 词在多少篇不同文档出现
    pub fn df(&self, term: &str) -> usize {
        let mut set = HashSet::new();

        for p in self.get(term) {
            set.insert(p.doc_id);
        }

        set.len()
    }
}
