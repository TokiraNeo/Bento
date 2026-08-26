/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use uuid::Uuid;

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_uuid_simple() -> String {
    Uuid::new_v4().as_simple().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_v4() {
        {
            let id = generate_uuid_simple();
            println!("uuid-v4: {}", id);
        }

        {
            let id = generate_uuid_simple();
            println!("uuid-v4-simple: {}", id);
        }
    }
}
