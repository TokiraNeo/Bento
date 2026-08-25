/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use bento_protocol::tool::{ToolDefinition, ToolSearchHit};
use serde::{Deserialize, Serialize};

pub(crate) type ToolDocId = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolDocField {
    Name,
    Tags,
    Description,
}

/// 词法通道的排序结果
#[derive(Debug)]
pub(crate) struct ScoredHit {
    pub doc_id: ToolDocId,
    pub score: f32,
}

pub(crate) struct SearchFields<'a> {
    pub name: &'a str,
    pub tags: &'a [String],
    pub description: &'a str,
}

#[derive(Clone)]
pub(crate) struct IndexedTool {
    pub name: String,
    pub namespace: String,
    pub definition: ToolDefinition,
}

impl IndexedTool {
    pub fn new(namespace: &str, definition: ToolDefinition) -> Self {
        Self {
            name: definition.name.clone(),
            namespace: namespace.to_string(),
            definition,
        }
    }

    pub fn to_hit(&self) -> ToolSearchHit {
        ToolSearchHit {
            qualified_name: format!("{}.{}", self.namespace, self.name),
            description: self.definition.description.clone(),
        }
    }

    pub fn search_fields(&self) -> SearchFields {
        SearchFields {
            name: &self.definition.name,
            tags: &self.definition.tags,
            description: &self.definition.description,
        }
    }
}
