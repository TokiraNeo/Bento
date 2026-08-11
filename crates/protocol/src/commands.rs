/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

pub mod tool_command {
    pub const TOOLS_REGISTER: &'static str = "tools.register";
    pub const TOOLS_REGISTERED: &'static str = "tools.registered";

    pub const TOOL_CALL: &'static str = "tool.call";
    pub const TOOL_RESULT: &'static str = "tool.result";
}

pub mod host_command {
    pub const HOST_HELLO: &'static str = "host.hello";
    pub const HOST_WELCOME: &'static str = "host.welcome";

    pub const HOST_READY: &'static str = "host.ready";

    //pub const HOST_PING: &'static str = "host.ping";
    //pub const HOST_PONG: &'static str = "host.pong";
}

pub mod env_command {
    pub const ENV_REPORT: &'static str = "env.report";
}

pub mod event_command {
    pub const EVENT_PUSH: &'static str = "event.push";
}
