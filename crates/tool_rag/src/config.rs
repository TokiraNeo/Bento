/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ToolRagConfig {
    pub top_k: u8, // 语义搜索候选数
    pub fts_k: u8, // FTS候选数
    pub semantic_search: bool,
}

impl Default for ToolRagConfig {
    fn default() -> Self {
        Self {
            top_k: 5,
            fts_k: 5,
            semantic_search: false,
        }
    }
}
