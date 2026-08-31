/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct HostServerConfig {
    pub host: String,
    pub port: u16,
    pub token: String,
}

impl Default for HostServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 2483,
            token: String::new(),
        }
    }
}
