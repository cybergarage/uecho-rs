// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use uecho::device::Device;
use uecho::object::Object;

pub struct TestDevice {
    obj: Object,
}

impl TestDevice {
    pub fn new() -> TestDevice {
        TestDevice { obj: Device::new() }
    }
}
