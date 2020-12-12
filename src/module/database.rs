// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

pub struct Database {
    path: String,
}

impl Database {
    pub fn new<S: Into<String>>(path: S) -> Database {
        Database { path: path.into() }
    }

    fn get_path(&self) -> String {
        format!("{}", self.path)
    }
}

#[test]
fn test_database() {
    let db: Database = Database::new("/etc/langmore/lang.db");

    assert_eq!(db.get_path(), "/etc/langmore/lang.db".to_string());
}
