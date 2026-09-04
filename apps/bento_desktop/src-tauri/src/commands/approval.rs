/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::state::BentoAppState;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Runtime, State};

pub(crate) fn plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("bento_approval_plugin")
        .setup(|app, api| Ok(()))
        .on_event(|app, event| {})
        .invoke_handler(tauri::generate_handler![respond_tool_approval])
        .build()
}

#[tauri::command(rename_all = "snake_case")]
fn respond_tool_approval(state: State<BentoAppState>, id: String, approval: bool) {
    state.approval_handler.resolve(&id, approval);
}
