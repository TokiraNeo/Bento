/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import { invoke } from "@tauri-apps/api/core";
import type { CoreConfig } from "@bridge/types/core_config";

export function getConfig(): Promise<CoreConfig> {
  return invoke<CoreConfig>("get_config");
}

export function saveConfig(config: CoreConfig): Promise<void> {
  return invoke<void>("save_config", { config });
}
