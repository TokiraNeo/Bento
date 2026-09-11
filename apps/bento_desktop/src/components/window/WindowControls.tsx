/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import { getCurrentWindow } from "@tauri-apps/api/window";
import styles from "./WindowControls.module.css";

const appWindow = () => getCurrentWindow();

export function WindowControls() {
  return (
    <div className={styles.cluster}>
      <button
        type="button"
        className={styles.control}
        title="最小化"
        onClick={() => void appWindow().minimize().catch(() => {})}
      >
        <svg width="12" height="12" viewBox="0 0 12 12" aria-hidden>
          <path fill="currentColor" d="M2 6.25h8v1H2z" />
        </svg>
      </button>
      <button
        type="button"
        className={styles.control}
        title="最大化"
        onClick={() => void appWindow().toggleMaximize().catch(() => {})}
      >
        <svg width="12" height="12" viewBox="0 0 12 12" aria-hidden>
          <path
            fill="none"
            stroke="currentColor"
            d="M3.25 3.25h5.5v5.5h-5.5z"
          />
        </svg>
      </button>
      <button
        type="button"
        className={`${styles.control} ${styles.close}`}
        title="关闭"
        onClick={() => void appWindow().close().catch(() => {})}
      >
        <svg width="12" height="12" viewBox="0 0 12 12" aria-hidden>
          <path
            fill="currentColor"
            d="M3.2 2.5 2.5 3.2 5.3 6 2.5 8.8l.7.7L6 6.7l2.8 2.8.7-.7L6.7 6l2.8-2.8-.7-.7L6 5.3z"
          />
        </svg>
      </button>
    </div>
  );
}
