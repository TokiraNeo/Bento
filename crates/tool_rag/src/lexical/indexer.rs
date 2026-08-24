/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod inverted;

use crate::ToolRagConfig;
use crate::lexical::LexicalTokenizer;
use crate::lexical::indexer::inverted::InvertedTable;
use crate::model::{IndexedTool, SearchFields, ToolDocField, ToolDocId};
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct LexicalIndexer {
    inverted_table: InvertedTable,
}

impl Default for LexicalIndexer {
    fn default() -> Self {
        Self {
            inverted_table: InvertedTable::new(),
        }
    }
}

impl LexicalIndexer {
    pub fn build(docs: &Vec<Arc<IndexedTool>>, config: &ToolRagConfig) -> Self {
        let inverted_table = Self::build_inverted_table(docs);

        Self { inverted_table }
    }

    fn build_inverted_table(docs: &Vec<Arc<IndexedTool>>) -> InvertedTable {
        let mut inverted_table = InvertedTable::new();

        for i in 0..docs.len() {
            let fields = docs[i].search_fields();

            // name
            inverted_table.insert(
                i,
                ToolDocField::Name,
                LexicalTokenizer::tokenize(fields.name),
            );

            // tags
            inverted_table.insert(
                i,
                ToolDocField::Tags,
                LexicalTokenizer::tokenize(fields.tags),
            );

            // description
            inverted_table.insert(
                i,
                ToolDocField::Description,
                LexicalTokenizer::tokenize(fields.description),
            );
        }

        inverted_table
    }
}
