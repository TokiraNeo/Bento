/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::events::core_events;
use crate::state::BentoAppState;
use bento_core::{CoreConfig, CoreEngine, CoreEvent};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn get_config(state: State<BentoAppState>) -> CoreConfig {
    state.config.read().unwrap().clone()
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn save_config(state: State<BentoAppState>, config: CoreConfig) {
    CoreConfig::write(&state.config_path, &config);
    *state.config.write().unwrap() = config;
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn start_engine(state: State<BentoAppState>) {
    let config = state.config.read().unwrap().clone();
    let approval_handler = state.approval_handler.clone();

    let engine = Arc::new(CoreEngine::new(config, approval_handler));

    {
        let runner = engine.clone();

        tauri::async_runtime::spawn(async move {
            if let Err(err) = runner.run().await {
                eprintln!("Bento Core Engine Interrupted: {}", err);
            }
        });
    }

    {
        let runner = engine.clone();
        let app = state.app.clone();

        tauri::async_runtime::spawn(handle_core_events(app, runner));
    }

    *state.engine.write().unwrap() = Some(engine);
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn stop_engine(state: State<BentoAppState>) {
    *state.engine.write().unwrap() = None;
}

async fn handle_core_events(app: AppHandle, engine: Arc<CoreEngine>) {
    let mut receiver = engine.subscribe();

    while let Ok(event) = receiver.recv().await {
        match event {
            CoreEvent::HostsChanged { hosts } => {
                let _ = app.emit(core_events::HOSTS_CHANGED, &hosts);
            }
        }
    }
}
