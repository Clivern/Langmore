// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use std::fs::read_to_string;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

// Writer type
pub struct Writer {}

// Writer type methods
impl Writer {
    ///
    /// Returns a writer object
    ///
    /// # Returns
    ///
    /// *  An instance of the writer object
    ///
    pub fn new() -> Writer {
        Writer {}
    }

    ///
    /// Check if file exists
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the file
    ///
    /// # Returns
    ///
    /// *  A boolean whether file exists or not
    ///
    pub fn file_exists(&self, path: String) -> bool {
        let path = Path::new(path.as_str());

        if path.exists() {
            true
        } else {
            false
        }
    }

    ///
    /// Write file content
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the file
    /// * `content` - The file content
    ///
    pub fn overwrite(&self, path: String, content: String) {
        let mut file = File::create(path.as_str()).unwrap();
        file.write_all(content.as_str().as_bytes()).unwrap();
    }

    ///
    /// Append to file content
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the file
    /// * `line` - The line content to append
    ///
    pub fn append(&self, path: String, line: String) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path.to_string())
            .unwrap();

        file.write_all(line.as_str().as_bytes()).unwrap();
    }

    ///
    /// Reads the file content
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the file
    ///
    /// # Returns
    ///
    /// *  The file content
    ///
    pub fn read(&self, path: String) -> String {
        let contents = read_to_string(path.to_string())
            .expect("Should have been able to read the file");
        format!("{}", contents)
    }
}

#[test]
fn test_writer() {
    let wt: Writer = Writer::new();

    assert_eq!(wt.file_exists("cache/.gitignore".to_string()), true);
    assert_eq!(wt.file_exists("cache/gitignore".to_string()), false);

    wt.overwrite("cache/test1.log".to_string(), "Hello World".to_string());
    assert_eq!(wt.file_exists("cache/test1.log".to_string()), true);

    wt.append("cache/test1.log".to_string(), "\nHello World".to_string());
    assert_eq!(wt.file_exists("cache/test1.log".to_string()), true);

    wt.append("cache/test2.log".to_string(), "Hello World".to_string());
    assert_eq!(wt.file_exists("cache/test2.log".to_string()), true);

    assert_eq!(
        wt.read("cache/test1.log".to_string()),
        "Hello World\nHello World".to_string()
    );
}
