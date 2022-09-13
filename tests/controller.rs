// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use uecho::controller::Controller;

pub struct TestController {
    obj: Controller,
}

impl TestController {
    pub fn new() -> TestController {
        TestController {
            obj: Controller::new(),
        }
    }
}
