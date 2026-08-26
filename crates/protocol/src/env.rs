/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// ```json
/// {
///   "payload": {}
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvReport {
    #[serde(default)]
    pub payload: Value,
}
