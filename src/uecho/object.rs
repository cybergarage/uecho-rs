// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::collections::HashMap;

use crate::uecho::property::*;
use crate::uecho::util::bytes::Bytes;

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

    pub fn property(&mut self, code: PropertyCode) -> Option<&mut Property> {
        self.properties.get_mut(&code)
    }

    pub fn property_attribute(&mut self, code: PropertyCode) -> Option<PropertyAttribute> {
        match self.property(code) {
            Some(prop) => return Some(prop.attribute()),
            None => return None,
        }
    }

    pub fn set_property_attribute(&mut self, code: PropertyCode, attr: PropertyAttribute) -> bool {
        match self.property(code).as_mut() {
            Some(prop) => {
                prop.set_attribute(attr);
                true
            }
            None => false,
        }
    }

    pub fn set_property_data(&mut self, code: PropertyCode, data: &[u8]) -> bool {
        match self.property(code).as_mut() {
            Some(prop) => {
                prop.set_data(data);
                true
            }
            None => false,
        }
    }

    pub fn set_property_byte(&mut self, code: PropertyCode, v: u8) -> bool {
        let data: &[u8] = &[v];
        self.set_property_data(code, data)
    }

    pub fn set_property_bytes(&mut self, code: PropertyCode, data: &[u8]) -> bool {
        self.set_property_data(code, data)
    }

    pub fn set_property_integer(&mut self, code: PropertyCode, val: u32, byte_size: usize) -> bool {
        let mut buf = Vec::<u8>::with_capacity(byte_size);
        Bytes::from_u32(val, &mut buf);
        self.set_property_data(code, &buf)
    }

    pub fn property_data(&mut self, code: PropertyCode) -> Option<&PropertyData> {
        match self.property(code) {
            Some(prop) => return Some(prop.data()),
            None => return None,
        }
    }

    pub fn equals_property_data(&mut self, code: PropertyCode, data: &[u8]) -> bool {
        match self.property(code) {
            Some(prop) => prop.equals_data(data),
            None => false,
        }
    }
}
