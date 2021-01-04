// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use bincode;
use bincode::serialize_into;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;

#[derive(Serialize, Debug, Deserialize)]
struct Database {
    path: String,
    data: HashMap<String, String>,
}

// Database type methods
impl Database {
    pub fn new<S: Into<String>>(path: S) -> Database {
        Database {
            path: path.into(),
            data: HashMap::new(),
        }
    }

    pub fn get_path(&self) -> String {
        format!("{}", self.path)
    }

    pub fn set<S: Into<String>>(&mut self, key: S, value: S) -> usize {
        self.data.insert(key.into(), value.into());
        self.data.len()
    }

    pub fn remove<S: Into<String>>(&mut self, key: S) -> usize {
        self.data.remove(&key.into());
        self.data.len()
    }

    pub fn get<S: Into<String>>(&self, key: S) -> String {
        match self.data.get(&key.into()) {
            Some(value) => value.to_string(),
            _ => String::new(),
        }
    }

    pub fn load(&self) -> Database {
        let f = File::open(self.path.as_str()).unwrap();
        let mut db: Database = Database::new(self.path.as_str());
        db.data = bincode::deserialize_from(&f).unwrap();
        db
    }

    pub fn flush(&self) {
        let mut f = File::create(self.path.as_str()).unwrap();
        serialize_into(&mut f, &self.data).unwrap();
        drop(f)
    }
}

#[test]
fn test_database() {
    let mut db: Database = Database::new("./cache/langmore.db");

    assert_eq!(db.get_path(), "./cache/langmore.db".to_string());

    assert_eq!(db.set("key1", "value1"), 1);
    assert_eq!(db.set("key2", "value2"), 2);
    assert_eq!(db.set("key3", "value3"), 3);

    assert_eq!(db.get("key1"), "value1".to_string());
    assert_eq!(db.get("key2"), "value2".to_string());
    assert_eq!(db.get("key3"), "value3".to_string());

    assert_eq!(db.remove("key1"), 2);
    assert_eq!(db.remove("key2"), 1);
    assert_eq!(db.remove("key3"), 0);

    assert_eq!(db.set("key1", "value1"), 1);
    assert_eq!(db.set("key2", "value2"), 2);
    assert_eq!(db.set("key3", "value3"), 3);

    db.flush();

    let db2: Database = db.load();

    assert_eq!(db2.get("key1"), "value1".to_string());
    assert_eq!(db2.get("key2"), "value2".to_string());
    assert_eq!(db2.get("key3"), "value3".to_string());
}
