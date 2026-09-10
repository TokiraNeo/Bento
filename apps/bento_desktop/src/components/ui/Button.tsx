/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import type { ReactNode } from "react";
import styles from "./Button.module.css";

type Tone = "primary" | "danger";

export function Button(props: {
  tone?: Tone;
  size?: "default" | "sm";
  disabled?: boolean;
  children: ReactNode;
  onClick?: () => void;
  type?: "button" | "submit";
}) {
  const tone = props.tone ?? "primary";
  const size = props.size ?? "default";

  return (
    <button
      type={props.type ?? "button"}
      className={`${styles.button} ${styles[tone]} ${size === "sm" ? styles.sm : ""}`}
      disabled={props.disabled}
      onClick={props.onClick}
    >
      {props.children}
    </button>
  );
}
