/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod approval;
mod commands;
mod state;

use bento_core::CoreConfig;
use state::BentoAppState;
use std::sync::{Arc, RwLock};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(commands::runtime::plugin())
        .setup(|app| {
            let config_path = app.path().app_config_dir()?.join("config.json");

            let config = CoreConfig::read(&config_path);

            app.manage(BentoAppState {
                config_path,
                config: RwLock::new(config),
                engine: RwLock::new(None),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
