/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod approval_task;

use crate::approval::approval_task::{ApprovalTask, ApprovalTaskManager};
use crate::events;
use async_trait::async_trait;
use bento_core::{ToolApprovalHandler, ToolApprovalRequest};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::oneshot;

pub(crate) struct BentoApprovalHandler {
    app: AppHandle,
    approval_manager: ApprovalTaskManager,
}

impl BentoApprovalHandler {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            approval_manager: ApprovalTaskManager::new(),
        }
    }

    pub fn resolve(&self, id: &str, approval: bool) -> bool {
        self.approval_manager.resolve(id, approval)
    }
}

#[async_trait]
impl ToolApprovalHandler for BentoApprovalHandler {
    async fn request(&self, request: ToolApprovalRequest) -> bool {
        let id = request.id.clone();

        let (sender, receiver) = oneshot::channel::<bool>();

        let task = ApprovalTask { responder: sender };
        self.approval_manager.register(id, task);

        // 唤起前端弹窗
        let _ = self
            .app
            .emit(events::approval::TOOL_APPROVAL_REQUEST, &request);

        match tokio::time::timeout(Duration::from_mins(1), receiver).await {
            Ok(Ok(approval)) => approval,

            _ => {
                self.approval_manager.cancel(&request.id);
                false
            }
        }
    }
}
