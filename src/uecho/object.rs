// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::collections::hash_map::Entry;
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

    pub fn set_property(&mut self, code: PropertyCode, attr: PropertyAttribute) -> bool {
        self.add_property(Property::new_with(code, attr))
    }

    pub fn property(&mut self, code: PropertyCode) -> Entry<'_, PropertyCode, Property> {
        self.properties.entry(code)
    }
}
