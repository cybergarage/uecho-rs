// Copyright (C) 2022 The uecho-rs Authors All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(dead_code)]

use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::protocol;
use crate::util::Bytes;

pub const PROPERTY_CODE_MIN: u8 = 0x80;
pub const PROPERTY_CODE_MAX: u8 = 0xFF;

pub const PROPERTY_MAP_FORMAT1_MAX_SIZE: i32 = 15;
pub const PROPERTY_MAP_FORMAT2_SIZE: i32 = 18;
pub const PROPERTY_MAP_FORMAT_MAX_SIZE: i32 = PROPERTY_MAP_FORMAT2_SIZE;

/// PropertyCode represents an ECHONET-Lite property code in an ECHONET-Lite property.
pub type PropertyCode = u8;

/// PropertyData represents an ECHONET-Lite property data in an ECHONET-Lite property.
pub type PropertyData = Vec<u8>;

/// PropertyEnumCode represents an ECHONET-Lite data (EDT) code in an ECHONET-Lite property.
pub type PropertyEnumCode = u32;

#[derive(Copy, Clone)]
/// PropertyRule represents an ECHONET-Lite property access rule for an ECHONET-Lite property.
pub enum PropertyRule {
    Prohibited = 0,
    Required = 1,
    Optional = 2,
}

/// PropertyEnum represents an ECHONET-Lite property enumerated data for an ECHONET-Lite property.
pub struct PropertyEnum {
    code: PropertyEnumCode,
    name: String,
    desc: String,
}

impl PropertyEnum {
    pub fn new() -> PropertyEnum {
        PropertyEnum {
            code: 0,
            name: String::from(""),
            desc: String::from(""),
        }
    }

    pub fn set_code(&mut self, code: PropertyEnumCode) -> &mut Self {
        self.code = code;
        self
    }

    pub fn code(&self) -> PropertyEnumCode {
        self.code
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_description(&mut self, desc: String) -> &mut Self {
        self.desc = desc;
        self
    }

    pub fn description(&self) -> &String {
        &self.name
    }
}

impl Clone for PropertyEnum {
    fn clone(&self) -> PropertyEnum {
        PropertyEnum {
            code: self.code,
            name: self.name.clone(),
            desc: self.desc.clone(),
        }
    }
}

impl Hash for PropertyEnum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

/// Each ECHONET-Lite object has properties. Property represents an ECHONET-Lite property in an ECHONET-Lite object.
pub struct Property {
    code: PropertyCode,
    data: PropertyData,
    name: String,
    typ: String,
    capacity: usize,
    read_attr: PropertyRule,
    write_attr: PropertyRule,
    anno_attr: PropertyRule,
    enums: HashMap<PropertyEnumCode, PropertyEnum>,
}

impl Property {
    pub fn new() -> Property {
        Property {
            code: 0,
            data: Vec::new(),
            name: String::from(""),
            typ: String::from(""),
            capacity: 0,
            read_attr: PropertyRule::Prohibited,
            write_attr: PropertyRule::Prohibited,
            anno_attr: PropertyRule::Prohibited,
            enums: HashMap::new(),
        }
    }

    pub fn set_code(&mut self, code: PropertyCode) -> &mut Self {
        self.code = code;
        self
    }

    pub fn code(&self) -> PropertyCode {
        self.code
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_data_type(&mut self, typ: String) -> &mut Self {
        self.typ = typ;
        self
    }

    pub fn data_type(&self) -> &String {
        &self.typ
    }

    pub fn set_capacity(&mut self, capacity: usize) -> &mut Self {
        self.capacity = capacity;
        self
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn set_read_attribute(&mut self, attr: PropertyRule) -> &mut Self {
        self.read_attr = attr;
        self
    }

    pub fn read_attribute(&self) -> PropertyRule {
        self.read_attr
    }

    pub fn set_write_attribute(&mut self, attr: PropertyRule) -> &mut Self {
        self.write_attr = attr;
        self
    }

    pub fn write_attribute(&self) -> PropertyRule {
        self.write_attr
    }
    pub fn set_anno_attribute(&mut self, attr: PropertyRule) -> &mut Self {
        self.anno_attr = attr;
        self
    }

    pub fn anno_attribute(&self) -> PropertyRule {
        self.anno_attr
    }

    pub fn is_read_required(&self) -> bool {
        match self.read_attr {
            PropertyRule::Prohibited => return false,
            PropertyRule::Required => return true,
            PropertyRule::Optional => return false,
        };
    }

    pub fn is_write_required(&self) -> bool {
        match self.write_attr {
            PropertyRule::Prohibited => return false,
            PropertyRule::Required => return true,
            PropertyRule::Optional => return false,
        };
    }

    pub fn is_announce_required(&self) -> bool {
        match self.anno_attr {
            PropertyRule::Prohibited => return false,
            PropertyRule::Required => return true,
            PropertyRule::Optional => return false,
        };
    }

    pub fn is_readable(&self) -> bool {
        match self.read_attr {
            PropertyRule::Prohibited => return false,
            PropertyRule::Required => return true,
            PropertyRule::Optional => return true,
        };
    }

    pub fn is_writable(&self) -> bool {
        match self.write_attr {
            PropertyRule::Prohibited => return false,
            PropertyRule::Required => return true,
            PropertyRule::Optional => return true,
        };
    }

    pub fn is_announceable(&self) -> bool {
        match self.anno_attr {
            PropertyRule::Prohibited => return false,
            PropertyRule::Required => return true,
            PropertyRule::Optional => return true,
        };
    }

    pub fn is_readonly(&self) -> bool {
        if !self.is_readable() {
            return false;
        }
        if self.is_writable() {
            return false;
        }
        true
    }

    pub fn is_writeonly(&self) -> bool {
        if !self.is_writable() {
            return false;
        }
        if self.is_readable() {
            return false;
        }
        true
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn set_data(&mut self, data: &[u8]) -> &mut Self {
        self.data = data.to_vec();
        self
    }

    pub fn set_byte_data(&mut self, v: u8) -> &mut Self {
        let data: &[u8] = &[v];
        self.set_data(data);
        self
    }

    pub fn set_bytes_data(&mut self, data: &[u8]) -> &mut Self {
        self.set_data(data);
        self
    }

    pub fn set_integer_data(&mut self, val: u32, byte_size: usize) -> &mut Self {
        let mut buf = Vec::<u8>::with_capacity(byte_size);
        Bytes::from_u32(val, &mut buf);
        self.set_data(&buf);
        self
    }

    pub fn add_data(&mut self, data: &[u8]) -> &mut Self {
        self.data.append(&mut data.to_vec());
        self
    }

    pub fn data(&self) -> &PropertyData {
        &self.data
    }

    pub fn byte_data(&self) -> u8 {
        if self.data.len() <= 0 {
            return 0;
        }
        return self.data[0];
    }

    pub fn integer_data(&self) -> u32 {
        if self.data.len() <= 0 {
            return 0;
        }
        return Bytes::to_u32(&self.data);
    }

    pub fn equals_data(&self, data: &[u8]) -> bool {
        if self.data.len() != data.len() {
            return false;
        }
        for n in 0..data.len() {
            if self.data[n] != data[n] {
                return false;
            }
        }
        true
    }

    pub fn add_enum(&mut self, e: PropertyEnum) -> bool {
        let code = e.code();
        self.enums.insert(code, e);
        true
    }

    pub fn enums(&self) -> Values<'_, PropertyEnumCode, PropertyEnum> {
        self.enums.values()
    }

    pub fn find_enum(&self, code: PropertyEnumCode) -> Option<&PropertyEnum> {
        self.enums.get(&code)
    }
}

impl Clone for Property {
    fn clone(&self) -> Property {
        let mut prop = Property {
            code: self.code,
            data: self.data.clone(),
            name: self.name.clone(),
            typ: self.typ.clone(),
            capacity: self.capacity,
            read_attr: self.read_attr,
            write_attr: self.write_attr,
            anno_attr: self.anno_attr,
            enums: HashMap::new(),
        };
        for e in self.enums() {
            prop.add_enum(e.clone());
        }
        prop
    }
}

impl Hash for Property {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code().hash(state);
    }
}

impl From<&Property> for protocol::Property {
    fn from(from: &Property) -> Self {
        protocol::Property::from(from.code, from.data.clone())
    }
}
