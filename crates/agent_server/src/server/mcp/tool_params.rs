/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use rmcp::schemars;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, schemars::JsonSchema)]
pub(super) struct SearchToolParams {
    pub text: String,
    pub top_k: usize,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub(super) struct GetToolSchemaParams {
    pub name: String,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub(super) struct CallToolParams {
    pub name: String,
    pub arguments: Value,
    pub timeout_ms: u64,
}
