/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use uuid::Uuid;

pub(super) fn create_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub(super) fn create_uuid_simple() -> String {
    Uuid::new_v4().simple().to_string()
}
