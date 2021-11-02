// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::util::bytes::Bytes;

pub const PROPERTY_CODE_MIN: u8 = 0x80;
pub const PROPERTY_CODE_MAX: u8 = 0xFF;

pub const PROPERTY_MAP_FORMAT1_MAX_SIZE: i32 = 15;
pub const PROPERTY_MAP_FORMAT2_SIZE: i32 = 18;
pub const PROPERTY_MAP_FORMAT_MAX_SIZE: i32 = PROPERTY_MAP_FORMAT2_SIZE;

pub const PROPERTY_ATTRIBUTE_NONE: u32 = 0x00;
pub const PROPERTY_ATTRIBUTE_READ: u32 = 0x01;
pub const PROPERTY_ATTRIBUTE_WRITE: u32 = 0x02;
pub const PROPERTY_ATTRIBUTE_ANNO: u32 = 0x10;
pub const PROPERTY_ATTRIBUTE_READ_WRITE: u32 = PROPERTY_ATTRIBUTE_READ | PROPERTY_ATTRIBUTE_WRITE;
pub const PROPERTY_ATTRIBUTE_READ_ANNO: u32 = PROPERTY_ATTRIBUTE_READ | PROPERTY_ATTRIBUTE_ANNO;

pub type PropertyCode = u8;
pub type PropertyAttribute = u32;
pub type PropertyData = Vec<u8>;

pub struct Property {
    code: PropertyCode,
    data: PropertyData,
    attr: PropertyAttribute,
}

impl Property {
    pub fn new() -> Property {
        Property::new_with(0, PROPERTY_ATTRIBUTE_NONE)
    }

    pub fn new_with(code: PropertyCode, attr: PropertyAttribute) -> Property {
        Property {
            code: code,
            data: Vec::new(),
            attr: attr,
        }
    }

    pub fn set_code(&mut self, code: PropertyCode) {
        self.code = code
    }

    pub fn code(&self) -> PropertyCode {
        self.code
    }

    pub fn set_attribute(&mut self, attr: PropertyAttribute) {
        self.attr = attr
    }

    pub fn attribute(&self) -> PropertyAttribute {
        self.attr
    }

    pub fn is_readable(&self) -> bool {
        if (self.attr & PROPERTY_ATTRIBUTE_READ) == 0 {
            return false;
        }
        true
    }

    pub fn is_writable(&self) -> bool {
        if (self.attr & PROPERTY_ATTRIBUTE_WRITE) == 0 {
            return false;
        }
        true
    }

    pub fn is_readonly(&self) -> bool {
        if (self.attr & PROPERTY_ATTRIBUTE_READ) == 0 {
            return false;
        }
        if (self.attr & PROPERTY_ATTRIBUTE_WRITE) != 0 {
            return false;
        }
        true
    }

    pub fn is_writeonly(&self) -> bool {
        if (self.attr & PROPERTY_ATTRIBUTE_WRITE) == 0 {
            return false;
        }
        if (self.attr & PROPERTY_ATTRIBUTE_READ) != 0 {
            return false;
        }
        true
    }

    pub fn is_announceable(&self) -> bool {
        if (self.attr & PROPERTY_ATTRIBUTE_ANNO) == 0 {
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
