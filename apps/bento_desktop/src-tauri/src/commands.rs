/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

pub(crate) mod approval;
pub(crate) mod hosts;
pub(crate) mod runtime;

macro_rules! command_handlers {
    () => {
        tauri::generate_handler![
            $crate::commands::approval::respond_tool_approval,
            $crate::commands::runtime::get_config,
            $crate::commands::runtime::save_config,
            $crate::commands::runtime::engine_running,
            $crate::commands::runtime::start_engine,
            $crate::commands::runtime::stop_engine,
            $crate::commands::hosts::list_hosts,
        ]
    };
}

pub(super) use command_handlers;
