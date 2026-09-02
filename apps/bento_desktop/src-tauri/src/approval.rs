/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use bento_core::{ToolApprovalHandler, ToolApprovalRequest};

pub(crate) struct BentoApprovalHandler;

impl BentoApprovalHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl ToolApprovalHandler for BentoApprovalHandler {
    async fn request(&self, request: ToolApprovalRequest) -> bool {
        todo!()
    }
}
