/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::ToolDocField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct ToolRagWeights {
    /// Name / Tags / Description 的 BM25F 字段权重
    pub fields: HashMap<ToolDocField, f32>,

    /// k1：词频饱和，越大越接近“出现几次就加几分”
    pub k1: f32,

    /// b：长度惩罚，0 忽略文档长短，1 完全按 dl/avgdl 归一
    pub b: f32,
}

impl Default for ToolRagWeights {
    fn default() -> Self {
        Self {
            fields: HashMap::from([
                (ToolDocField::Name, 3.0),
                (ToolDocField::Description, 2.0),
                (ToolDocField::Tags, 1.5),
            ]),
            k1: 1.2,
            b: 0.3,
        }
    }
}

impl ToolRagWeights {
    pub fn field_weight(&self, field: ToolDocField) -> f32 {
        self.fields.get(&field).copied().unwrap_or(1.0)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ToolRagConfig {
    pub weights: ToolRagWeights,

    /// 词法召回截断条数
    pub candidate: usize,
}

impl Default for ToolRagConfig {
    fn default() -> Self {
        Self {
            weights: ToolRagWeights::default(),
            candidate: 3,
        }
    }
}
