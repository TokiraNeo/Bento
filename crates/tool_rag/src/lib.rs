/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod catalog;
pub mod config;
mod engine;
mod fusion;
mod model;
mod retrieve;
mod snapshot;

pub use config::ToolRagConfig;
pub use engine::ToolRagEngine;
pub use fusion::FusionConfig;
pub use model::ToolDocField;
pub use retrieve::{ExactRetrieveConfig, LexicalRetrieveConfig, SemanticRetrieveConfig};
