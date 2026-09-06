/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::event::HostHandler;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum HostSessionState {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct HostMeta {
    pub name: String,
    pub namespace: String,
    pub state: HostSessionState,
    pub tool_count: usize,
}

impl Default for HostMeta {
    fn default() -> Self {
        Self {
            name: String::new(),
            namespace: String::new(),
            state: HostSessionState::Connecting,
            tool_count: 0,
        }
    }
}

pub(crate) struct HostSession {
    pub session_id: String,
    pub handler: HostHandler,
    meta: RwLock<HostMeta>,
}

impl HostSession {
    pub fn new(session_id: String, handler: HostHandler) -> Self {
        Self {
            session_id,
            handler,
            meta: RwLock::new(HostMeta::default()),
        }
    }

    pub fn update_meta(&self, f: impl FnOnce(&mut HostMeta)) {
        let mut meta = self.meta.write().unwrap();
        f(&mut meta);
    }

    pub fn get_meta(&self) -> HostMeta {
        self.meta.read().unwrap().clone()
    }
}

/// session-id - HostSession
type HostMap = HashMap<String, Arc<HostSession>>;

#[derive(Clone)]
pub(crate) struct HostSessionManager {
    hosts: Arc<RwLock<HostMap>>,
}

impl HostSessionManager {
    pub fn new() -> Self {
        Self {
            hosts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, session: Arc<HostSession>) {
        self.hosts
            .write()
            .unwrap()
            .insert(session.session_id.clone(), session);
    }

    pub fn remove(&self, session_id: &str) {
        self.hosts.write().unwrap().remove(session_id);
    }

    pub fn handler(&self, session_id: &str) -> Option<HostHandler> {
        self.hosts
            .read()
            .unwrap()
            .get(session_id)
            .map(|s| s.handler.clone())
    }

    pub fn list_hosts(&self) -> Vec<HostMeta> {
        let mut hosts: Vec<HostMeta> = self
            .hosts
            .read()
            .unwrap()
            .values()
            .map(|s| s.get_meta())
            .collect();

        hosts.sort_by(|a, b| a.namespace.cmp(&b.namespace));
        hosts
    }
}
