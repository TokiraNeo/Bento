/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import styles from "./Badge.module.css";

type Tone = "success" | "accent" | "muted" | "warning" | "danger";

export function Badge(props: {
  tone: Tone;
  children: React.ReactNode;
}) {
  return (
    <span className={`${styles.badge} ${styles[props.tone]}`}>
      {props.children}
    </span>
  );
}
