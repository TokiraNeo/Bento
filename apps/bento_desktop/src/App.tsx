/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import { useState } from "react";
import { ConfigPanel } from "@components/config/ConfigPanel";
import { HostsPanel } from "@components/host/HostsPanel";
import { ApprovalOverlay } from "@components/approval/ApprovalOverlay";
import { useToolApproval } from "@bridge/events/approval_events";
import { EngineToggle } from "@components/engine/EngineToggle";
import styles from "./App.module.css";

type Tab = "hosts" | "tools" | "config" | "logs";

const TABS: { key: Tab; label: string }[] = [
  { key: "hosts", label: "Hosts" },
  { key: "tools", label: "Tools" },
  { key: "config", label: "Config" },
  { key: "logs", label: "Logs" },
];

function App() {
  const [tab, setTab] = useState<Tab>("config");
  const { pending, respond } = useToolApproval();

  return (
    <div className={styles.shell}>
      <div className={`iridescent ${styles.iridescentLayer}`} aria-hidden />
      <div className={`chrome ${styles.window}`}>
        <header className={styles.topbar}>
          <div className={styles.brand}>Bento</div>
          <nav className={styles.nav}>
            {TABS.map((t) => (
              <button
                key={t.key}
                className={`${styles.navItem} ${tab === t.key ? `${styles.navItemActive} glow` : ""}`}
                onClick={() => setTab(t.key)}
              >
                {t.label}
              </button>
            ))}
          </nav>
          <div className={styles.topbarEnd}>
            <EngineToggle />
          </div>
        </header>
        <main className={styles.content}>
          {tab === "config" && <ConfigPanel />}
          {tab === "hosts" && <HostsPanel />}
          {tab === "tools" && (
            <span className={styles.placeholder}>Tools</span>
          )}
          {tab === "logs" && (
            <span className={styles.placeholder}>Logs</span>
          )}
        </main>
      </div>

      {pending.length > 0 && (
        <ApprovalOverlay
          current={pending[0]}
          total={pending.length}
          onRespond={respond}
        />
      )}
    </div>
  );
}

export default App;
