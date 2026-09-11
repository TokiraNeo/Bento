/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::events::{core_events, engine_events};
use crate::state::BentoAppState;
use bento_core::{CoreConfig, CoreEngine, CoreEvent, HostMeta};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};

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
pub(crate) fn engine_running(state: State<BentoAppState>) -> bool {
    state.engine.read().unwrap().is_some()
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn start_engine(state: State<BentoAppState>) {
    let config = state.config.read().unwrap().clone();
    let approval_handler = state.approval_handler.clone();
    let app = state.app.clone();

    let mut slot = state.engine.write().unwrap();
    if slot.is_some() {
        return;
    }

    let engine = Arc::new(CoreEngine::new(config, approval_handler));

    {
        let runner = engine.clone();
        let app = app.clone();

        tauri::async_runtime::spawn(async move {
            if let Err(err) = runner.run().await {
                eprintln!("Bento Core Engine Interrupted: {}", err);
            }

            if let Some(app_state) = app.try_state::<BentoAppState>() {
                let mut slot = app_state.engine.write().unwrap();
                if slot
                    .as_ref()
                    .is_some_and(|current| Arc::ptr_eq(current, &runner))
                {
                    *slot = None;
                }
            }

            let _ = app.emit(engine_events::ENGINE_STATUS, false);
            let _ = app.emit(core_events::HOSTS_CHANGED, &Vec::<HostMeta>::new());
        });
    }

    {
        let runner = engine.clone();
        let app = app.clone();
        tauri::async_runtime::spawn(handle_core_events(app, runner));
    }

    *slot = Some(engine);
    drop(slot);
    let _ = app.emit(engine_events::ENGINE_STATUS, true);
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn stop_engine(state: State<BentoAppState>) {
    let engine = state.engine.write().unwrap().take();
    if let Some(engine) = engine {
        engine.stop();
    }
    let _ = state.app.emit(engine_events::ENGINE_STATUS, false);
    let _ = state.app.emit(core_events::HOSTS_CHANGED, &Vec::<HostMeta>::new());
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
