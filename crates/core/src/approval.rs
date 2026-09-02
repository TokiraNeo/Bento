/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct ToolApprovalRequest {
    pub id: String,
    pub qualified_name: String,
    pub namespace: String,
    pub tool_name: String,
    pub arguments: Value,
}

#[async_trait]
pub trait ToolApprovalHandler: Send + Sync {
    async fn request(&self, request: ToolApprovalRequest) -> bool;
}
