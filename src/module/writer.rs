// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use std::path::Path;

pub struct Writer {

}

impl Writer {
    pub fn new() -> Writer {
        Writer {}
    }

	pub fn file_exists(&self, path: String) -> bool {
	    let path = Path::new(path.as_str());

	    if path.exists() {
	        true
	    } else {
	       	false
	    }
	}
}

#[test]
fn test_writer() {
    let writer: Writer = Writer::new();

    assert_eq!(writer.file_exists("cache/.gitignore".to_string()), true);
    assert_eq!(writer.file_exists("cache/gitignore".to_string()), false);
}
