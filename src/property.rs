// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::util::bytes::Bytes;

pub const PROPERTY_CODE_MIN: u8 = 0x80;
pub const PROPERTY_CODE_MAX: u8 = 0xFF;

pub const PROPERTY_MAP_FORMAT1_MAX_SIZE: i32 = 15;
pub const PROPERTY_MAP_FORMAT2_SIZE: i32 = 18;
pub const PROPERTY_MAP_FORMAT_MAX_SIZE: i32 = PROPERTY_MAP_FORMAT2_SIZE;

pub type PropertyCode = u8;
pub type PropertyData = Vec<u8>;

#[derive(Copy, Clone)]
pub enum PropertyAttr {
    Prohibited = 0,
    Required = 1,
    Optional = 2,
}

pub struct Property {
    code: PropertyCode,
    data: PropertyData,
    name: String,
    read_attr: PropertyAttr,
    write_attr: PropertyAttr,
    anno_attr: PropertyAttr,
}

impl Property {
    pub fn new() -> Property {
        Property::new_with(0)
    }

    pub fn new_with(code: PropertyCode) -> Property {
        Property {
            code: code,
            data: Vec::new(),
            name: String::from(""),
            read_attr: PropertyAttr::Prohibited,
            write_attr: PropertyAttr::Prohibited,
            anno_attr: PropertyAttr::Prohibited,
        }
    }

    pub fn set_code(&mut self, code: PropertyCode) {
        self.code = code
    }

    pub fn code(&self) -> PropertyCode {
        self.code
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_read_attribute(&mut self, attr: PropertyAttr) {
        self.read_attr = attr
    }

    pub fn read_attribute(&self) -> PropertyAttr {
        self.read_attr
    }

    pub fn set_write_attribute(&mut self, attr: PropertyAttr) {
        self.write_attr = attr
    }

    pub fn write_attribute(&self) -> PropertyAttr {
        self.write_attr
    }
    pub fn set_anno_attribute(&mut self, attr: PropertyAttr) {
        self.anno_attr = attr
    }

    pub fn anno_attribute(&self) -> PropertyAttr {
        self.anno_attr
    }

    pub fn is_readable(&self) -> bool {
        match self.read_attr {
            PropertyAttr::Prohibited => false,
            PropertyAttr::Required => true,
            PropertyAttr::Optional => true,
        };
        false
    }

    pub fn is_writable(&self) -> bool {
        match self.write_attr {
            PropertyAttr::Prohibited => false,
            PropertyAttr::Required => true,
            PropertyAttr::Optional => true,
        };
        false
    }

    pub fn is_announceable(&self) -> bool {
        match self.anno_attr {
            PropertyAttr::Prohibited => false,
            PropertyAttr::Required => true,
            PropertyAttr::Optional => true,
        };
        false
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

    pub fn set_data(&mut self, data: &[u8]) -> bool {
        self.data = data.to_vec();
        true
    }

    pub fn set_byte_data(&mut self, v: u8) -> bool {
        let data: &[u8] = &[v];
        self.set_data(data)
    }

    pub fn set_bytes_data(&mut self, data: &[u8]) -> bool {
        self.set_data(data)
    }

    pub fn set_integer_data(&mut self, val: u32, byte_size: usize) -> bool {
        let mut buf = Vec::<u8>::with_capacity(byte_size);
        Bytes::from_u32(val, &mut buf);
        self.set_data(&buf)
    }

    pub fn add_data(&mut self, data: &[u8]) -> bool {
        self.data.append(&mut data.to_vec());
        true
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
}
