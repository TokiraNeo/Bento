/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use bento_protocol::tool::ToolDefinition;
use std::borrow::Cow;
use tokio::sync::{mpsc, oneshot};

pub struct ToolIndexTask {
    pub namespace: String,
    pub tools: Vec<ToolDefinition>,
    pub responder: oneshot::Sender<Result<usize, Cow<'static, str>>>,
}

#[derive(Clone)]
pub struct ToolIndexRequester(pub mpsc::Sender<ToolIndexTask>);

impl ToolIndexRequester {
    pub async fn send(&self, req: ToolIndexTask) {
        let _ = self.0.send(req).await;
    }
}
