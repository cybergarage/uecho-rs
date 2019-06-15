// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub struct Property {
    pub code: [u8; 3],
}

impl Property {
    pub fn new() -> Property {
        Property { code: [0, 0, 0] }
    }

    pub fn get_size(&self) -> u64 {
        5
    }
}
