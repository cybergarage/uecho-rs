// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::object::Object;
use crate::property::*;

pub const OBJECT_OPERATING_STATUS: u8 = 0x80;
pub const OBJECT_MANUFACTURER_CODE: u8 = 0x8A;
pub const OBJECT_ANNO_PROPERTY_MAP: u8 = 0x9D;
pub const OBJECT_SET_PROPERTY_MAP: u8 = 0x9E;
pub const OBJECT_GET_PROPERTY_MAP: u8 = 0x9F;

pub const OBJECT_OPERATING_STATUS_ON: u8 = 0x30;
pub const OBJECT_OPERATING_STATUS_OFF: u8 = 0x31;
pub const OBJECT_OPERATING_STATUS_SIZE: usize = 1;
pub const OBJECT_MANUFACTURER_EVALUATION_CODE_MIN: u32 = 0xFFFFF0;
pub const OBJECT_MANUFACTURER_EVALUATION_CODE_MAX: u32 = 0xFFFFFF;
pub const OBJECT_MANUFACTURER_CODE_SIZE: usize = 3;
pub const OBJECT_PROPERTY_MAP_MAX_SIZE: usize = 16;
pub const OBJECT_ANNO_PROPERTY_MAP_MAX_SIZE: usize = OBJECT_PROPERTY_MAP_MAX_SIZE + 1;
pub const OBJECT_SET_PROPERTY_MAP_MAX_SIZE: usize = OBJECT_PROPERTY_MAP_MAX_SIZE + 1;
pub const OBJECT_GET_PROPERTY_MAP_MAX_SIZE: usize = OBJECT_PROPERTY_MAP_MAX_SIZE + 1;

pub const OBJECT_MANUFACTURER_UNKNOWN: u32 = OBJECT_MANUFACTURER_EVALUATION_CODE_MIN;

impl Object {
    pub fn set_manufacturer_code(&mut self, code: u32) -> bool {
        self.set_property_integer(
            OBJECT_MANUFACTURER_CODE,
            code,
            OBJECT_MANUFACTURER_CODE_SIZE,
        )
    }

    pub fn manufacturer_code(&mut self) -> &mut Property {
        self.property(OBJECT_MANUFACTURER_CODE).unwrap()
    }
}
