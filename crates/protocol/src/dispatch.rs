/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::jsonrpc::{JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};

pub enum InboundFrame {
    Request(JsonRpcRequest),
    Notification(JsonRpcNotification),
}

pub enum OutboundFrame {
    Response(JsonRpcResponse),
    Notification(JsonRpcNotification),
}
