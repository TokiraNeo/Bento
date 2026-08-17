/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::tool::ToolDefinition;

/// RAG Indexer for tool.
#[async_trait::async_trait]
pub trait ToolIndexer {
    async fn index_tools(namespace: String, tools: Vec<ToolDefinition>);
}
