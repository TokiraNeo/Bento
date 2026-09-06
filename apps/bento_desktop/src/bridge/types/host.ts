/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

export enum HostSessionState {
  Connecting,
  Helloed,
  Registered,
  Ready,
  Closed,
}

export interface HostMeta {
  name: string;
  namespace: string;
  state: HostSessionState;
  tool_count: number;
}
