/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

pub(crate) mod exact;
pub(crate) mod lexical;
pub(crate) mod semantic;

pub use exact::ExactRetrieveConfig;
pub use lexical::LexicalRetrieveConfig;
pub use semantic::SemanticRetrieveConfig;
