/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import type { HostMeta, HostSessionState } from "@bridge/types/host";
import { Badge } from "@components/ui/Badge";
import styles from "./HostCard.module.css";

const STATE_TONE: Record<HostSessionState, "success" | "accent" | "muted" | "warning" | "danger"> = {
  Ready: "success",
  Registered: "accent",
  Helloed: "muted",
  Connecting: "warning",
  Closed: "danger",
};

const HOST_STATE_LABEL: Record<HostSessionState, string> = {
  Connecting: "连接中",
  Helloed: "已握手",
  Registered: "已注册",
  Ready: "就绪",
  Closed: "已断开",
};

export function HostCard(props: { host: HostMeta }) {
  const { host } = props;

  return (
    <div className={`iridescent-border glow-hover ${styles.card}`}>
      <div className={styles.port} aria-hidden />

      <div className={styles.head}>
        <span className={styles.namespace}>{host.namespace}</span>
        <Badge tone={STATE_TONE[host.state]}>{HOST_STATE_LABEL[host.state]}</Badge>
      </div>

      <div className={styles.name}>{host.name || "—"}</div>

      <div className={styles.foot}>
        <span className={styles.toolCount}>{host.tool_count} tools</span>
      </div>
    </div>
  );
}
