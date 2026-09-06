/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

export interface ToolApprovalRequest {
  id: string;
  qualified_name: string;
  namespace: string;
  tool_name: string;
  arguments: Record<string, any>; // Json Value
}
