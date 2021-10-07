// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub const PropertyCodeMin: u8 = 0x80;
pub const PropertyCodeMax: u8 = 0xFF;

pub const PropertyMapFormat1MaxSize: i32 = 15;
pub const PropertyMapFormat2Size: i32 = 18;
pub const PropertyMapFormatMaxSize: i32 = PropertyMapFormat2Size;

pub const PropertyAttributeNone: u32 = 0x00;
pub const PropertyAttributeRead: u32 = 0x01;
pub const PropertyAttributeWrite: u32 = 0x02;
pub const PropertyAttributeAnno: u32 = 0x10;
pub const PropertyAttributeReadWrite: u32 = PropertyAttributeRead | PropertyAttributeWrite;
pub const PropertyAttributeReadAnno: u32 = PropertyAttributeRead | PropertyAttributeAnno;

pub type PropertyCode = u8;
pub type PropertyAttribute = u32;

pub struct Property {
    code: PropertyCode,
    data: Vec<u8>,
    attr: PropertyAttribute,
}

impl Property {
    pub fn new() -> Property {
        Property::new_with(0, PropertyAttributeNone)
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
        if (self.attr & PropertyAttributeRead) == 0 {
            return false;
        }
        true
    }

    pub fn is_writable(&self) -> bool {
        if (self.attr & PropertyAttributeWrite) == 0 {
            return false;
        }
        true
    }

    pub fn is_readonly(&self) -> bool {
        if (self.attr & PropertyAttributeRead) == 0 {
            return false;
        }
        if (self.attr & PropertyAttributeWrite) != 0 {
            return false;
        }
        true
    }

    pub fn is_writeonly(&self) -> bool {
        if (self.attr & PropertyAttributeWrite) == 0 {
            return false;
        }
        if (self.attr & PropertyAttributeRead) != 0 {
            return false;
        }
        true
    }

    pub fn is_announceable(&self) -> bool {
        if (self.attr & PropertyAttributeAnno) == 0 {
            return false;
        }
        true
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.data = data.to_vec()
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
