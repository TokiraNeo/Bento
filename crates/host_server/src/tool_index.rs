/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use async_trait::async_trait;
use bento_protocol::tool::ToolDefinition;
use std::borrow::Cow;

#[async_trait]
pub trait ToolIndexSink: Send + Sync {
    async fn replace(
        &self,
        session_id: &str,
        namespace: &str,
        tools: Vec<ToolDefinition>,
    ) -> Result<usize, Cow<'static, str>>;

    async fn ready(&self, session_id: &str) -> Result<(), Cow<'static, str>>;

    async fn remove(&self, session_id: &str) -> Result<(), Cow<'static, str>>;
}
