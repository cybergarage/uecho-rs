// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::object::*;
use crate::uecho::super_object::*;

pub const PROFILE_CLASS_GROUP_CODE: u8 = 0x0E;

pub const PROFILE_FAULT_STATUS: u8 = 0x88;
pub const PROFILE_MANUFACTURER_CODE: u8 = OBJECT_MANUFACTURER_CODE;
pub const PROFILE_PLACE_OF_BUSINESS_CODE: u8 = 0x8B;
pub const PROFILE_PRODUCT_CODE: u8 = 0x8C;
pub const PROFILE_SERIAL_NUMBER: u8 = 0x8D;
pub const PROFILE_DATE_OF_MANUFACTURE: u8 = 0x8E;
pub const PROFILE_ANNO_PROPERTY_MAP: u8 = OBJECT_ANNO_PROPERTY_MAP;
pub const PROFILE_SET_PROPERTY_MAP: u8 = OBJECT_SET_PROPERTY_MAP;
pub const PROFILE_GET_PROPERTY_MAP: u8 = OBJECT_GET_PROPERTY_MAP;

pub const PROFILE_FAULT_STATUS_LEN: i32 = 1;
pub const PROFILE_MANUFACTURER_CODE_LEN: usize = OBJECT_MANUFACTURER_CODE_SIZE;
pub const PROFILE_PLACE_OF_BUSINESS_CODE_LEN: i32 = 3;
pub const PROFILE_PRODUCT_CODE_LEN: i32 = 12;
pub const PROFILE_SERIAL_NUMBER_LEN: i32 = 12;
pub const PROFILE_DATE_OF_MANUFACTURE_LEN: i32 = 4;

pub const PROFILE_FAULT_ENCOUNTERED: u8 = 0x41;
pub const PROFILE_NO_FAULT_ENCOUNTERED: u8 = 0x42;
pub const PROFILE_MANUFACTURER_UNKNOWN: u32 = OBJECT_MANUFACTURER_UNKNOWN;

pub struct Profile {}

impl Profile {
    pub fn new() -> Object {
        Object::new()
    }
}
