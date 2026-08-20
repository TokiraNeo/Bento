/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod config;
mod event;
mod namespace;
mod request_task;
mod server;
mod session;
mod tool_index;

pub use config::HostServerConfig;
pub use server::HostServer;
pub use tool_index::ToolIndexSink;
