/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import { useEffect, useRef } from "react";
import { HostMeta } from "@bridge/types/host";
import { listen } from "@tauri-apps/api/event";

type OnChangedCallback = (hosts: HostMeta[]) => void;

export function useHosts(onChanged?: OnChangedCallback) {
  const onChangedRef = useRef(onChanged);
  onChangedRef.current = onChanged;

  useEffect(() => {
    const unlisten = listen<HostMeta[]>("hosts-changed", (event) => {
      onChangedRef.current?.(event.payload);
    });

    return () => {
      unlisten.then((fn) => {
        fn();
      });
    };
  }, []);

  return {  };
}
