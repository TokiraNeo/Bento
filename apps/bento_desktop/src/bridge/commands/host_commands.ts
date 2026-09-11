/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import { invoke } from "@tauri-apps/api/core";
import type { HostMeta } from "@bridge/types/host";

export async function listHosts(): Promise<HostMeta[]> {
  return invoke<HostMeta[]>("list_hosts");
}
