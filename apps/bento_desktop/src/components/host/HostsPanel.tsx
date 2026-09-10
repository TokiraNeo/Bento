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
        <div className={styles.empty}>等待宿主连接…</div>
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
