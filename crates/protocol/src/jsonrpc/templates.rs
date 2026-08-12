/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::error::ErrorCode;
use crate::jsonrpc::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use serde::{Deserialize, Serialize};

pub struct TJsonRpcRequest<P: Serialize + Deserialize> {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: P,
}

impl<To: Serialize + Deserialize> TryFrom<JsonRpcRequest> for TJsonRpcRequest<To> {
    type Error = JsonRpcError;

    fn try_from(value: JsonRpcRequest) -> Result<Self, Self::Error> {
        let to = serde_json::from_value::<To>(value.params).map_err(|err| JsonRpcError {
            code: ErrorCode::ParseError.code(),
            message: err.to_string(),
            payload: None,
        })?;

        Ok(Self {
            jsonrpc: value.jsonrpc,
            id: value.id,
            method: value.method,
            params: to,
        })
    }
}

pub struct TJsonRpcNotification<P: Serialize + Deserialize> {
    pub jsonrpc: String,
    pub method: String,
    pub params: P,
}

impl<To: Serialize + Deserialize> TryFrom<JsonRpcNotification> for TJsonRpcNotification<To> {
    type Error = JsonRpcError;

    fn try_from(value: JsonRpcNotification) -> Result<Self, Self::Error> {
        let to = serde_json::from_value::<To>(value.params).map_err(|err| JsonRpcError {
            code: ErrorCode::ParseError.code(),
            message: err.to_string(),
            payload: None,
        })?;

        Ok(Self {
            jsonrpc: value.jsonrpc,
            method: value.method,
            params: to,
        })
    }
}

pub struct TJsonRpcResponse<P: Serialize + Deserialize> {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<P>,
    pub error: Option<JsonRpcError>,
}

impl<To: Serialize + Deserialize> TryFrom<JsonRpcResponse> for TJsonRpcResponse<To> {
    type Error = JsonRpcError;

    fn try_from(value: JsonRpcResponse) -> Result<Self, Self::Error> {
        if let Some(err) = value.error {
            return Ok(Self {
                jsonrpc: value.jsonrpc,
                id: value.id,
                result: None,
                error: Some(err),
            });
        }

        let to = serde_json::from_value::<To>(value.result.unwrap()).map_err(|err| JsonRpcError {
            code: ErrorCode::ParseError.code(),
            message: err.to_string(),
            payload: None,
        })?;

        Ok(Self {
            jsonrpc: value.jsonrpc,
            id: value.id,
            result: to,
            error: None,
        })
    }
}