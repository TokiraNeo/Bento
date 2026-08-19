/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::config::ToolRagConfig;

pub struct ToolRagEngine {
    config: ToolRagConfig,
}

impl ToolRagEngine {
    pub fn new(config: ToolRagConfig) -> Self {
        Self { config }
    }
}