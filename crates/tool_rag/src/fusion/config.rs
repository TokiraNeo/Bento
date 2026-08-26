/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use serde::{Deserialize, Serialize};

/// 跨通道融合：Weighted RRF。
#[derive(Clone, Serialize, Deserialize)]
pub struct FusionConfig {
    /// RRF 平滑常数，常用 60
    pub rrf_k: f32,
    pub exact: f32,
    pub lexical: f32,
    pub semantic: f32,
}

impl Default for FusionConfig {
    fn default() -> Self {
        Self {
            rrf_k: 60.0,
            exact: 2.0,
            lexical: 1.0,
            semantic: 1.0,
        }
    }
}
