// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub struct Manager {}

impl Manager {
    pub fn new() -> Manager {
        Manager {}
    }

    pub fn start(&self) -> bool {
        true
    }

    pub fn stop(&self) -> bool {
        true
    }
}
