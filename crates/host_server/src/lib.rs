/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

pub mod config;
mod event;
mod namespace;
mod request_task;
pub mod server;
mod session;
pub mod tool_index;
mod utilities;

pub use config::HostServerConfig;
pub use server::HostServer;