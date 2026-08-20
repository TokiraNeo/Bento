/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolRisk {
    Low,
    #[default]
    Medium,
    High,
}

/// ```json
/// {
///   "name": "my_tool",
///   "description": "...",
///   "input_schema": {
///     "type": "object",
///     "properties": {
///       "...": { "type": "...", "description": "..."}
///     }
///   },
///   "risk": "medium",
///   "domain": "Modeling",
///   "tags": ["tag1", "tag2"],
///   "example": "my_tool(...)"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,

    /// ```json
    /// "input_schema": {
    ///   "type": "object",
    ///   "properties": {
    ///       "...": { "type": "...", "description": "..."}
    ///   }
    /// }
    /// ```
    pub input_schema: Value,

    #[serde(default)]
    pub risk: ToolRisk,

    #[serde(default)]
    pub domain: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub example: Option<String>,
}

impl ToolDefinition {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Tool name cannot be empty".into());
        }

        if self.description.trim().is_empty() {
            return Err("Tool description cannot be empty".into());
        }

        if !self.input_schema.is_object() {
            return Err("Tool input_schema must be a JSON object".into());
        }

        Ok(())
    }
}

/// ```json
/// { "type": "text", "text": "..."}
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

/// `bento.search_tools` 的查询参数。
#[derive(Clone, Serialize, Deserialize)]
pub struct ToolSearchQuery {
    pub text: String,
    pub top_k: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

/// `bento.search_tools` 返回的工具。
#[derive(Clone, Serialize, Deserialize)]
pub struct ToolSearchHit {
    pub qualified_name: String,
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}
