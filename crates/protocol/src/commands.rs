/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

pub mod tool_command {
    pub static TOOLS_REGISTER: &'static str = "tools.register";
    pub static TOOLS_REGISTERED: &'static str = "tools.registered";

    pub static TOOL_CALL: &'static str = "tool.call";
    pub static TOOL_RESULT: &'static str = "tool.result";
}

pub mod host_command {
    pub static HOST_HELLO: &'static str = "host.hello";
    pub static HOST_WELCOME: &'static str = "host.welcome";

    pub static HOST_READY: &'static str = "host.ready";

    pub static HOST_PING: &'static str = "host.ping";
    pub static HOST_PONG: &'static str = "host.pong";
}

pub mod env_command {
    pub static ENV_REPORT: &'static str = "env.report";
}

pub mod event_command {
    pub static EVENT_PUSH: &'static str = "event.push";
}
