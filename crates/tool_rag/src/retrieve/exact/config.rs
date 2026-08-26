/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use serde::{Deserialize, Serialize};

/// 精确通道：整串匹配 `qualified_name`。
#[derive(Clone, Serialize, Deserialize)]
pub struct ExactRetrieveConfig {
    /// 精确命中最多返回几条（同名冲突时）
    pub candidate: usize,
}

impl Default for ExactRetrieveConfig {
    fn default() -> Self {
        Self { candidate: 4 }
    }
}
