// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::node_profile::*;

pub struct Object {}

pub type ObjectCode = u32;

impl Object {
    pub fn new() -> Object {
        Object {}
    }
}

pub fn object_is_profile_object_code(code: ObjectCode) -> bool {
    if code == NodeProfileObject {
        return true;
    }
    if code == NodeProfileObjectReadOnly {
        return true;
    }
    return false;
}
