/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::approval::BentoApprovalHandler;
use bento_core::{CoreConfig, CoreEngine};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub(crate) struct BentoAppState {
    pub config_path: PathBuf,
    pub config: RwLock<CoreConfig>,
    pub engine: RwLock<Option<Arc<CoreEngine>>>,
    pub approval_handler: Arc<BentoApprovalHandler>,
}
