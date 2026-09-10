/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import { invoke } from "@tauri-apps/api/core";

export async function engineRunning(): Promise<boolean> {
  return invoke<boolean>("engine_running");
}

export async function startEngine(): Promise<void> {
  return invoke<void>("start_engine");
}

export async function stopEngine(): Promise<void> {
  return invoke<void>("stop_engine");
}
