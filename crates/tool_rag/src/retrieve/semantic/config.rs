/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use serde::{Deserialize, Serialize};

/// 语义检索通道配置
#[derive(Clone, Serialize, Deserialize)]
pub struct SemanticRetrieveConfig {
    pub enabled: bool,

    /// 语义召回截断条数
    pub candidate: usize,
}

impl Default for SemanticRetrieveConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            candidate: 32,
        }
    }
}
