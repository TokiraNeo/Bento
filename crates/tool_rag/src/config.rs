/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ToolRagConfig {
    pub candidate: u8,         // 工具候选数
    pub top_k: u8,             // 语义搜索候选数
    pub rrf_k: u8,             //
    pub semantic_search: bool, // 是否启用语义搜索
}

impl Default for ToolRagConfig {
    fn default() -> Self {
        Self {
            candidate: 5,
            top_k: 3,
            rrf_k: 3,
            semantic_search: false,
        }
    }
}
