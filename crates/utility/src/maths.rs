/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

pub fn normalize(vector: Vec<f32>) -> Vec<f32> {
    if vector.is_empty() {
        return vec![];
    }

    let norm: f32 = vector
        .iter()
        .fold(0.0, |sum, num| sum + num.powf(2.0))
        .sqrt();

    vector.iter().map(|num| num / norm).collect()
}

pub fn dot(v1: &[f32], v2: &[f32]) -> f32 {
    if v1.is_empty() || v2.is_empty() {
        return 0.0;
    }

    if v1.len() != v2.len() {
        return 0.0;
    }

    v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum()
}
