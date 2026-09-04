/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use bento_protocol::jsonrpc::JsonRpcResponse;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

pub(super) enum RequestOutcome {
    Response(JsonRpcResponse),
    Canceled,
}

pub(super) struct RequestTask {
    pub id: String,
    pub session_id: String,
    pub responder: oneshot::Sender<RequestOutcome>,
}

#[derive(Clone)]
pub(super) struct RequestTaskManager {
    pending_task: Arc<Mutex<HashMap<String, RequestTask>>>,
}

impl RequestTaskManager {
    pub fn new() -> Self {
        Self {
            pending_task: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, task: RequestTask) {
        let id = task.id.clone();
        self.pending_task.lock().unwrap().insert(id, task);
    }

    pub fn response(&self, response: JsonRpcResponse) -> bool {
        let id = response.id.as_str();

        let task = self.pending_task.lock().unwrap().remove(id);

        match task {
            None => false,

            Some(task) => match task.responder.send(RequestOutcome::Response(response)) {
                Ok(_) => true,
                Err(_) => false,
            },
        }
    }

    pub fn cancel(&self, id: &str) -> bool {
        let task = self.pending_task.lock().unwrap().remove(id);

        match task {
            Some(t) => {
                let _ = t.responder.send(RequestOutcome::Canceled);
                true
            }
            None => false,
        }
    }

    pub fn cancel_all_for_session(&self, session_id: &str) {
        let mut map = self.pending_task.lock().unwrap();

        let ids: Vec<String> = map
            .iter()
            .filter(|(_, task)| task.session_id == session_id)
            .map(|(id, _)| id.clone())
            .collect();

        for id in ids {
            if let Some(t) = map.remove(&id) {
                let _ = t.responder.send(RequestOutcome::Canceled);
            }
        }
    }
}
