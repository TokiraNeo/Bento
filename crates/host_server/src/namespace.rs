/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use bento_utility::generate_uuid_simple;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// namespace - session_id
type HostNamespaceMap = HashMap<String, String>;

#[derive(Clone)]
pub(super) struct HostNamespaceRegistry {
    map: Arc<Mutex<HostNamespaceMap>>,
}

impl HostNamespaceRegistry {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, host_name: String, session_id: String) -> String {
        let mut map = self.map.lock().unwrap();

        if !map.contains_key(&host_name) {
            let namespace = host_name.clone();
            map.insert(host_name.clone(), session_id);
            return namespace;
        }

        let mut n = 2;
        while n < 20 {
            let candidate = format!("{}#{}", host_name, n);
            if !map.contains_key(&candidate) {
                map.insert(candidate.clone(), session_id);
                return candidate;
            }
            n += 1;
        }

        // 通常来说，不会走到这里。
        let namespace = format!("{}#{}", host_name, generate_uuid_simple());
        map.insert(namespace.clone(), session_id);
        namespace
    }

    pub fn release(&self, namespace: String) {
        self.map.lock().unwrap().remove(&namespace);
    }

    pub fn session_id(&self, namespace: String) -> Option<String> {
        self.map.lock().unwrap().get(&namespace).cloned()
    }
}
