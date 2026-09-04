/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

pub(super) struct ApprovalTask {
    pub responder: oneshot::Sender<bool>,
}

pub(super) struct ApprovalTaskManager {
    pending_task: Arc<Mutex<HashMap<String, ApprovalTask>>>,
}

impl ApprovalTaskManager {
    pub fn new() -> Self {
        Self {
            pending_task: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, id: String, task: ApprovalTask) {
        self.pending_task.lock().unwrap().insert(id, task);
    }

    pub fn cancel(&self, id: &str) {
        self.pending_task.lock().unwrap().remove(id);
    }

    pub fn resolve(&self, id: &str, approval: bool) -> bool {
        let task = self.pending_task.lock().unwrap().remove(id);

        match task {
            None => false,

            Some(task) => match task.responder.send(approval) {
                Err(_) => false,
                Ok(_) => true,
            },
        }
    }
}
