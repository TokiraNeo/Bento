/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::error::ErrorCode;
use crate::jsonrpc::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 有 `id` → Request；无 `id` → Notification。
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InboundFrame {
    Request(JsonRpcRequest),
    Notification(JsonRpcNotification),
}

/// 有 `id` → Response；无 `id` → Notification。
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutboundFrame {
    Response(JsonRpcResponse),
    Notification(JsonRpcNotification),
}

pub fn parse_inbound_frame(text: &str) -> Result<InboundFrame, JsonRpcError> {
    let value: Value = serde_json::from_str(text).map_err(|err| JsonRpcError {
        code: ErrorCode::ParseError.code(),
        message: format!("Failed to parse JSON: {}", err),
        payload: None,
    })?;

    if !value.is_object() {
        return Err(JsonRpcError {
            code: ErrorCode::InvalidRequest.code(),
            message: "Invalid JsonRpc Request.".into(),
            payload: None,
        });
    }

    if value.get("id").is_some() {
        let request = serde_json::from_value::<JsonRpcRequest>(value)
            .map(InboundFrame::Request)
            .map_err(|_| JsonRpcError {
                code: ErrorCode::InvalidRequest.code(),
                message: "Invalid JsonRpc Request.".into(),
                payload: None,
            })?;

        return Ok(request);
    }

    let notification = serde_json::from_value::<JsonRpcNotification>(value)
        .map(InboundFrame::Notification)
        .map_err(|_| JsonRpcError {
            code: ErrorCode::InvalidRequest.code(),
            message: "Invalid JsonRpc Notification.".into(),
            payload: None,
        })?;

    Ok(notification)
}
