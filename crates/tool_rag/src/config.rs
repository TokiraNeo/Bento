/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::fusion::FusionConfig;
use crate::retrieve::{ExactRetrieveConfig, LexicalRetrieveConfig, SemanticRetrieveConfig};
use serde::{Deserialize, Serialize};

/// 工具检索总配置
#[derive(Clone, Serialize, Deserialize)]
pub struct ToolRagConfig {
    pub exact: ExactRetrieveConfig,
    pub lexical: LexicalRetrieveConfig,
    pub semantic: SemanticRetrieveConfig,
    pub fusion: FusionConfig,
}

impl Default for ToolRagConfig {
    fn default() -> Self {
        Self {
            exact: ExactRetrieveConfig::default(),
            lexical: LexicalRetrieveConfig::default(),
            semantic: SemanticRetrieveConfig::default(),
            fusion: FusionConfig::default(),
        }
    }
}
