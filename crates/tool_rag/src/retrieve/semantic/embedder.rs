/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use std::borrow::Cow;

pub type EmbedVector = Vec<f32>;

pub trait Embedder: Send + Sync {
    fn embed_docs(&self, docs: &[String]) -> Result<Vec<EmbedVector>, Cow<'static, str>>;
    fn embed_query(&self, query: &str) -> Result<EmbedVector, Cow<'static, str>>;
}
