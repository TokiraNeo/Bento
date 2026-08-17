/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::error::ErrorCode;
use crate::jsonrpc::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

pub fn from_request<P: DeserializeOwned>(
    from: JsonRpcRequest,
) -> Result<TJsonRpcRequest<P>, JsonRpcError> {
    TJsonRpcRequest::<P>::try_from(from)
}

pub fn from_notification<P: DeserializeOwned>(
    from: JsonRpcNotification,
) -> Result<TJsonRpcNotification<P>, JsonRpcError> {
    TJsonRpcNotification::<P>::try_from(from)
}

pub fn from_response<P: DeserializeOwned>(
    from: JsonRpcResponse,
) -> Result<TJsonRpcResponse<P>, JsonRpcError> {
    TJsonRpcResponse::<P>::try_from(from)
}

pub fn into_request<P: Serialize>(
    from: TJsonRpcRequest<P>,
) -> Result<JsonRpcRequest, JsonRpcError> {
    JsonRpcRequest::try_from(from)
}

pub fn into_notification<P: Serialize>(
    from: TJsonRpcNotification<P>,
) -> Result<JsonRpcNotification, JsonRpcError> {
    JsonRpcNotification::try_from(from)
}

pub fn into_response<P: Serialize>(
    from: TJsonRpcResponse<P>,
) -> Result<JsonRpcResponse, JsonRpcError> {
    JsonRpcResponse::try_from(from)
}

pub struct TJsonRpcRequest<P> {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: P,
}

impl<P: DeserializeOwned> TryFrom<JsonRpcRequest> for TJsonRpcRequest<P> {
    type Error = JsonRpcError;

    fn try_from(value: JsonRpcRequest) -> Result<Self, Self::Error> {
        let to = serde_json::from_value::<P>(value.params).map_err(|err| JsonRpcError {
            code: ErrorCode::ParseError.code(),
            message: Cow::Owned(err.to_string()),
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

impl<P: Serialize> TryFrom<TJsonRpcRequest<P>> for JsonRpcRequest {
    type Error = JsonRpcError;

    fn try_from(value: TJsonRpcRequest<P>) -> Result<Self, Self::Error> {
        let TJsonRpcRequest {
            jsonrpc,
            id,
            method,
            params,
        } = value;

        let from = serde_json::to_value(params).map_err(|err| JsonRpcError {
            code: ErrorCode::ParseError.code(),
            message: Cow::Owned(err.to_string()),
            payload: None,
        })?;

        Ok(Self {
            jsonrpc,
            id,
            method,
            params: from,
        })
    }
}

pub struct TJsonRpcNotification<P> {
    pub jsonrpc: String,
    pub method: String,
    pub params: P,
}

impl<P: DeserializeOwned> TryFrom<JsonRpcNotification> for TJsonRpcNotification<P> {
    type Error = JsonRpcError;

    fn try_from(value: JsonRpcNotification) -> Result<Self, Self::Error> {
        let to = serde_json::from_value::<P>(value.params).map_err(|err| JsonRpcError {
            code: ErrorCode::ParseError.code(),
            message: Cow::Owned(err.to_string()),
            payload: None,
        })?;

        Ok(Self {
            jsonrpc: value.jsonrpc,
            method: value.method,
            params: to,
        })
    }
}

impl<P: Serialize> TryFrom<TJsonRpcNotification<P>> for JsonRpcNotification {
    type Error = JsonRpcError;

    fn try_from(value: TJsonRpcNotification<P>) -> Result<Self, Self::Error> {
        let TJsonRpcNotification {
            jsonrpc,
            method,
            params,
        } = value;

        let from = serde_json::to_value(params).map_err(|err| JsonRpcError {
            code: ErrorCode::ParseError.code(),
            message: Cow::Owned(err.to_string()),
            payload: None,
        })?;

        Ok(Self {
            jsonrpc,
            method,
            params: from,
        })
    }
}

pub struct TJsonRpcResponse<P> {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<P>,
    pub error: Option<JsonRpcError>,
}

impl<P: DeserializeOwned> TryFrom<JsonRpcResponse> for TJsonRpcResponse<P> {
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

        match value.result {
            Some(v) => {
                let to = serde_json::from_value::<P>(v).map_err(|err| JsonRpcError {
                    code: ErrorCode::ParseError.code(),
                    message: Cow::Owned(err.to_string()),
                    payload: None,
                })?;

                Ok(Self {
                    jsonrpc: value.jsonrpc,
                    id: value.id,
                    result: Some(to),
                    error: None,
                })
            }
            None => Err(JsonRpcError {
                code: ErrorCode::InvalidParams.code(),
                message: "Got None params.".into(),
                payload: None,
            }),
        }
    }
}

impl<P: Serialize> TryFrom<TJsonRpcResponse<P>> for JsonRpcResponse {
    type Error = JsonRpcError;

    fn try_from(value: TJsonRpcResponse<P>) -> Result<Self, Self::Error> {
        let TJsonRpcResponse {
            jsonrpc,
            id,
            result,
            error,
        } = value;

        if let Some(err) = error {
            return Ok(Self {
                jsonrpc,
                id,
                result: None,
                error: Some(err),
            });
        }

        match result {
            Some(p) => {
                let from = serde_json::to_value(p).map_err(|err| JsonRpcError {
                    code: ErrorCode::ParseError.code(),
                    message: Cow::Owned(err.to_string()),
                    payload: None,
                })?;

                Ok(Self {
                    jsonrpc,
                    id,
                    result: Some(from),
                    error: None,
                })
            }

            None => Err(JsonRpcError {
                code: ErrorCode::InvalidParams.code(),
                message: "Got None result.".into(),
                payload: None,
            }),
        }
    }
}
