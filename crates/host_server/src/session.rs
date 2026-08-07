/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostRuntimeIdentity {
    pub host_name: String,
    pub host_version: String,
    pub host_id: String,
}

pub enum HostSessionState {
    /// 已建立 TCP,尚未收到合法 host.hello
    Connecting,
    /// hello 通过,已分配 session_id / namespace
    Helloed,
    /// 已注册工具
    Registered,
    /// host.ready 完成,可接收 tool.call
    Ready,
    /// 断开
    Closed,
}

pub struct HostSession {
    pub session_id: String,
    pub namespace: String,
    pub identity: HostRuntimeIdentity,
    pub state: HostSessionState,
}
