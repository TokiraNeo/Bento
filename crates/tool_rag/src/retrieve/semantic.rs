/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod config;
mod embedder;
mod indexer;

pub use config::SemanticRetrieveConfig;
pub use embedder::{EmbedVector, Embedder};
pub(crate) use indexer::SemanticIndexer;
