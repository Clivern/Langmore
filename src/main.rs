// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

mod util;

fn main() {
    println!("Hello from {}!", util::environ::configs::get_config("CARGO_PKG_NAME"));
}
