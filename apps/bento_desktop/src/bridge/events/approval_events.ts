/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import { ToolApprovalRequest } from "@bridge/types/tool_approval";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export function toolApproval() {
  const [pending, setPending] = useState<ToolApprovalRequest[]>([]);

  useEffect(() => {
    const unlisten = listen<ToolApprovalRequest>(
      "tool-approval-request",
      (event) => {
        setPending((queue) => [...queue, event.payload]);
      },
    );

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const respond = (id: string, approval: boolean) => {
    invoke("respond_tool_approval", { id, approval });
    setPending((queue) => queue.filter((req) => req.id !== id));
  };

  return { pending, respond };
}
