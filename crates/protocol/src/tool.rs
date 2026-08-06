/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolRisk { Low, Medium, High }

fn default_risk() -> ToolRisk { ToolRisk::Medium }

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

    #[serde(default = "default_risk")]
    pub risk: ToolRisk,

    #[serde(default)]
    pub domain: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub example: Option<String>
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRegisterRequest {
    pub tools: Vec<ToolDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRegisterResponse {
    pub count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
    pub call_id: String,
    pub tool_name: String,
    pub arguments: Value,
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

/// ```json
/// {
///   "call_id": "...",
///   "content": [
///     { "type": "text", "text": "..."}
///   ],
///   "is_error": false
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub call_id: String,
    pub content: Vec<ToolCallContent>,

    #[serde(default)]
    pub is_error: bool,
}
