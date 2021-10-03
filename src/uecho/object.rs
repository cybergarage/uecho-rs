// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::collections::HashMap;

use crate::uecho::property::*;

pub struct Object {
    properties: HashMap<PropertyCode, Property>,
}

pub type ObjectCode = u32;

impl Object {
    pub fn new() -> Object {
        Object {
            properties: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, prop: Property) -> bool {
        let code = prop.code();
        self.properties.insert(code, prop);
        true
    }
}
