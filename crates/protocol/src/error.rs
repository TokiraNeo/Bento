/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ErrorCode {
    #[error("Parse error")]
    ParseError = -32700,
    #[error("Invalid request")]
    InvalidRequest = -32600,
    #[error("Invalid params")]
    InvalidParams = -32602,
    #[error("Internal error")]
    InternalError = -32603,
    #[error("Host error")]
    HostError = -32000,
    #[error("Tool not found")]
    ToolNotFound = -32001,
    #[error("Tool execution failed")]
    ToolExecutionFailed = -32002,
    #[error("Tool timeout")]
    ToolTimeout = -32003,
    #[error("Invalid tool arguments")]
    InvalidToolArgs = -32010,
}

impl ErrorCode {
    pub fn code(&self) -> i32 {
        *self as i32
    }
}
