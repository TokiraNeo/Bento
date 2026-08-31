/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::state::BentoAppState;
use bento_core::{CoreConfig, CoreEngine};
use std::sync::Arc;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Runtime, State, generate_handler};

pub(crate) fn plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("bento_runtime_plugin")
        .setup(|app, api| {
            // Init Plugin here
            Ok(())
        })
        .on_event(|app, event| {
            // Handler event here
        })
        .invoke_handler(generate_handler![
            get_config,
            save_config,
            start_engine,
            stop_engine,
        ])
        .build()
}

#[tauri::command]
fn get_config(state: State<BentoAppState>) -> CoreConfig {
    state.config.read().unwrap().clone()
}

#[tauri::command]
fn save_config(state: State<BentoAppState>, config: CoreConfig) {
    CoreConfig::write(&state.config_path, &config);
    *state.config.write().unwrap() = config;
}

#[tauri::command]
fn start_engine(state: State<BentoAppState>) {
    let config = state.config.read().unwrap().clone();

    let engine = Arc::new(CoreEngine::new(config));
    let runner = engine.clone();

    *state.engine.write().unwrap() = Some(engine);

    tauri::async_runtime::spawn(async move {
        if let Err(err) = runner.run().await {
            eprintln!("Bento Core Engine Interrupted: {}", err);
        }
    });
}

#[tauri::command]
fn stop_engine(state: State<BentoAppState>) {
    *state.engine.write().unwrap() = None;
}
