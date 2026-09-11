/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::state::BentoAppState;
use bento_core::HostMeta;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn list_hosts(state: State<BentoAppState>) -> Vec<HostMeta> {
    match state.engine.read().unwrap().as_ref() {
        Some(engine) => engine.list_hosts(),
        None => Vec::new(),
    }
}
