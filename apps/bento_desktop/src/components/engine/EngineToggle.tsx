/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import {
  engineRunning,
  startEngine,
  stopEngine,
} from "@bridge/commands/runtime_commands";
import { Badge } from "@components/ui/Badge";
import { Button } from "@components/ui/Button";
import styles from "./EngineToggle.module.css";

export function EngineToggle() {
  const [running, setRunning] = useState(false);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    engineRunning()
      .then(setRunning)
      .catch(() => setRunning(false));

    const unlisten = listen<boolean>("engine-status", (event) => {
      setRunning(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const toggle = async () => {
    setBusy(true);
    try {
      if (running) {
        await stopEngine();
        setRunning(false);
      } else {
        await startEngine();
        setRunning(true);
      }
    } catch (error) {
      console.error(error);
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className={styles.cluster}>
      <Badge tone={running ? "success" : "muted"}>
        {running ? "运行中" : "已停止"}
      </Badge>
      <Button
        size="sm"
        tone={running ? "danger" : "primary"}
        disabled={busy}
        onClick={() => void toggle()}
      >
        {running ? "停止" : "启动"}
      </Button>
    </div>
  );
}
