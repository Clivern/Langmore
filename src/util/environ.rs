// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

// configs module
pub mod configs {
    // Get config var
    pub fn get_config(key: &str) -> String {
        std::env::var(key).unwrap()
    }
}

#[test]
fn test_get_config() {
    assert_eq!(configs::get_config("CARGO_PKG_NAME"), "dwal");
}
