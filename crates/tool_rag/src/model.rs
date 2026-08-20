/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use bento_protocol::tool::{ToolDefinition, ToolSearchHit};

pub(crate) type ToolDocId = usize;

pub(crate) struct ScoredHit {
    pub doc_id: ToolDocId,
    pub score: f64,
}

pub(crate) struct SearchFields<'a> {
    pub name: &'a str,
    pub tags: &'a [String],
    pub domain: Option<&'a str>,
    pub description: &'a str,
    pub example: Option<&'a str>,
}

#[derive(Clone)]
pub(crate) struct IndexedTool {
    pub qualified_name: String,
    pub namespace: String,
    pub definition: ToolDefinition,
}

impl IndexedTool {
    pub fn new(namespace: &str, definition: ToolDefinition) -> Self {
        Self {
            qualified_name: format!("{}.{}", namespace, definition.name.as_str()),
            namespace: namespace.to_string(),
            definition,
        }
    }

    pub fn to_hit(&self) -> ToolSearchHit {
        ToolSearchHit {
            qualified_name: self.qualified_name.clone(),
            description: self.definition.description.clone(),
            domain: self.definition.domain.clone(),
        }
    }

    pub fn search_fields(&self) -> SearchFields {
        SearchFields {
            name: &self.definition.name,
            tags: &self.definition.tags,
            domain: self.definition.domain.as_deref(),
            description: &self.definition.description,
            example: self.definition.example.as_deref(),
        }
    }
}
