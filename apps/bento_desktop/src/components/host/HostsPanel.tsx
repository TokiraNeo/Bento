/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import { useState } from "react";
import type { HostMeta } from "@bridge/types/host";
import { useHosts } from "@bridge/events/host_events";
import { HostCard } from "./HostCard";
import styles from "./HostsPanel.module.css";

export function HostsPanel() {
  const [hosts, setHosts] = useState<HostMeta[]>([]);

  useHosts(setHosts);

  return (
    <div className={styles.panel}>
      {hosts.length === 0 ? (
        <div className={styles.empty}>
          <p className={styles.emptyTitle}>等待宿主连接…</p>
          <p className={styles.emptyHint}>宿主插件经 WebSocket 连入后会自动出现在这里</p>
        </div>
      ) : (
        <div className={styles.grid}>
          {hosts.map((host) => (
            <HostCard key={host.namespace} host={host} />
          ))}
        </div>
      )}
    </div>
  );
}
