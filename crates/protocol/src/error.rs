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
    #[error("Method not found")]
    MethodNotFound = -32601,
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
    #[error("Approval required / denied")]
    ApprovalRequired = -32004,
    #[error("Task cancelled")]
    TaskCancelled = -32005,
    #[error("Host busy")]
    HostBusy = -32006,
    #[error("Auth failed")]
    AuthFailed = -32007,
    #[error("Protocol version mismatch")]
    ProtocolVersionMismatch = -32008,
    #[error("Host not ready")]
    HostNotReady = -32009,
    #[error("Invalid tool arguments")]
    InvalidToolArgs = -32010,
}

impl ErrorCode {
    pub fn code(&self) -> i32 {
        *self as i32
    }
}
