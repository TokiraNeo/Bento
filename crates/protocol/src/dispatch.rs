/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::error::ErrorCode;
use crate::jsonrpc::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
/// - 有 `id` + 有 `method` → Request
/// - 有 `id` + 无 `method` → Response
/// - 无 `id` + 有 `method` → Notification
///
/// untagged 尝试顺序：先 Request（同时要求 id 和 method），再 Response（只要求 id），
/// 最后 Notification（只要求 method）。
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Frame {
    Request(JsonRpcRequest),
    Response(JsonRpcResponse),
    Notification(JsonRpcNotification),
}

/// Host -> Bento
pub type InboundFrame = Frame;

/// Bento -> Host
pub type OutboundFrame = Frame;

pub fn parse_frame(text: &str) -> Result<Frame, JsonRpcError> {
    let value: Value = serde_json::from_str(text).map_err(|err| JsonRpcError {
        code: ErrorCode::ParseError.code(),
        message: Cow::Owned(format!("Failed to parse JSON: {}", err)),
        payload: None,
    })?;

    if !value.is_object() {
        return Err(jsonrpc_error(
            ErrorCode::InvalidRequest,
            "Invalid JsonRpc message.".into(),
        ));
    }

    let has_id = value.get("id").is_some();
    let has_method = value.get("method").is_some();

    let frame = match (has_id, has_method) {
        (true, true) => serde_json::from_value::<JsonRpcRequest>(value)
            .map(Frame::Request)
            .map_err(|_| {
                jsonrpc_error(
                    ErrorCode::InvalidRequest,
                    "Failed to parse JsonRpcRequest".into(),
                )
            })?,
        (true, false) => serde_json::from_value::<JsonRpcResponse>(value)
            .map(Frame::Response)
            .map_err(|_| {
                jsonrpc_error(
                    ErrorCode::InvalidRequest,
                    "Failed to parse JsonRpcResponse".into(),
                )
            })?,
        (false, true) => serde_json::from_value::<JsonRpcNotification>(value)
            .map(Frame::Notification)
            .map_err(|_| {
                jsonrpc_error(
                    ErrorCode::InvalidRequest,
                    "Failed to parse JsonRpcNotification".into(),
                )
            })?,
        (false, false) => {
            return Err(jsonrpc_error(
                ErrorCode::InvalidRequest,
                "Failed to parse JsonRpc".into(),
            ));
        }
    };

    Ok(frame)
}

fn jsonrpc_error(code: ErrorCode, message: Cow<'static, str>) -> JsonRpcError {
    JsonRpcError {
        code: code.code(),
        message,
        payload: None,
    }
}
