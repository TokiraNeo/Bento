/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { listHosts } from "@bridge/commands/host_commands";
import type { HostMeta } from "@bridge/types/host";

export function useHosts() {
  const [hosts, setHosts] = useState<HostMeta[]>([]);

  useEffect(() => {
    listHosts()
      .then(setHosts)
      .catch(() => setHosts([]));

    const unlisten = listen<HostMeta[]>("hosts-changed", (event) => {
      setHosts(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return hosts;
}
