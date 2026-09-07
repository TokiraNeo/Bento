/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import styles from "./Field.module.css";

interface FieldProps<T extends string | number> {
  label: string;
  value: T;
  onChange: (v: T) => void;
  type?: "text" | "number";
}

export function Field<T extends string | number>(props: FieldProps<T>) {
  const handleChange = (raw: string) => {
    if (props.type === "number") {
      props.onChange(Number(raw) as T);
    } else {
      props.onChange(raw as T);
    }
  };

  return (
    <label className={styles.field}>
      <span className={styles.label}>{props.label}</span>
      <input
        type={props.type ?? "text"}
        className={styles.input}
        value={props.value}
        onChange={(e) => handleChange(e.currentTarget.value)}
      />
    </label>
  );
}
