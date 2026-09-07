/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::state::BentoAppState;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn respond_tool_approval(state: State<BentoAppState>, id: String, approval: bool) {
    state.approval_handler.resolve(&id, approval);
}
