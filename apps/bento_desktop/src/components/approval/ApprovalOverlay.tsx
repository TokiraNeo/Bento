/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import type { ToolApprovalRequest } from "@bridge/types/tool_approval";
import { Button } from "@components/ui/Button";
import styles from "./ApprovalOverlay.module.css";

export function ApprovalOverlay(props: {
  current: ToolApprovalRequest;
  total: number;
  onRespond: (id: string, approval: boolean) => void;
}) {
  const { current, total, onRespond } = props;

  const args = JSON.stringify(current.arguments, null, 2);

  return (
    <div className={styles.backdrop}>
      <div className={`chrome ${styles.dialog}`}>
        <header className={styles.header}>
          <div className={styles.headerText}>
            <span className={styles.kicker}>工具调用审批</span>
            <h2 className={styles.name}>{current.qualified_name}</h2>
          </div>
          <span className={styles.queue}>
            {total > 1 ? `其中 ${total} 个待审批` : ""}
          </span>
        </header>

        <div className={styles.meta}>
          <span className={styles.metaLabel}>namespace</span>
          <span className={`${styles.metaValue} mono`}>{current.namespace}</span>
        </div>

        <div className={styles.argsBlock}>
          <span className={styles.metaLabel}>arguments</span>
          <pre className={`${styles.args} mono`}>{args}</pre>
        </div>

        <footer className={styles.footer}>
          <Button tone="danger" onClick={() => onRespond(current.id, false)}>
            拒绝
          </Button>
          <Button onClick={() => onRespond(current.id, true)}>批准</Button>
        </footer>
      </div>
    </div>
  );
}
