/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};

use crate::event::HostHandler;

#[derive(Default, Clone, Copy, Debug)]
pub(super) enum HostSessionState {
    /// 已建立 TCP,尚未收到合法 host.hello
    #[default]
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

pub(super) struct HostSession {
    pub session_id: String,
    pub state: HostSessionState,
    pub handler: HostHandler,
    pub namespace: String,
}

impl HostSession {
    pub fn new(session_id: String, handler: HostHandler) -> Self {
        Self {
            session_id,
            state: HostSessionState::Connecting,
            handler,
            namespace: String::new(),
        }
    }
}
