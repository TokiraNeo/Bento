/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use bento_protocol::tool::ToolDefinition;
use std::borrow::Cow;
use tokio::sync::{mpsc, oneshot};

pub enum ToolIndexTask {
    Replace {
        session_id: String,
        namespace: String,
        tools: Vec<ToolDefinition>,
        responder: oneshot::Sender<Result<usize, Cow<'static, str>>>,
    },
    Ready {
        session_id: String,
    },
    Remove {
        session_id: String,
    },
}

#[derive(Clone)]
pub struct ToolIndexRequester(mpsc::Sender<ToolIndexTask>);

impl ToolIndexRequester {
    pub fn new(sender: mpsc::Sender<ToolIndexTask>) -> Self {
        ToolIndexRequester(sender)
    }

    pub async fn send(&self, req: ToolIndexTask) -> Result<(), Cow<'static, str>> {
        let result = self.0.send(req).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Cow::Owned(err.to_string())),
        }
    }
}
