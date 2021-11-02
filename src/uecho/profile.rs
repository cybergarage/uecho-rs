// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::object::*;
use crate::uecho::super_object::*;

pub const ProfileClassGroupCode: u8 = 0x0E;

pub const ProfileFaultStatus: u8 = 0x88;
pub const ProfileManufacturerCode: u8 = ObjectManufacturerCode;
pub const ProfilePlaceOfBusinessCode: u8 = 0x8B;
pub const ProfileProductCode: u8 = 0x8C;
pub const ProfileSerialNumber: u8 = 0x8D;
pub const ProfileDateOfManufacture: u8 = 0x8E;
pub const ProfileAnnoPropertyMap: u8 = ObjectAnnoPropertyMap;
pub const ProfileSetPropertyMap: u8 = ObjectSetPropertyMap;
pub const ProfileGetPropertyMap: u8 = ObjectGetPropertyMap;

pub const ProfileFaultStatusLen: i32 = 1;
pub const ProfileManufacturerCodeLen: usize = ObjectManufacturerCodeSize;
pub const ProfilePlaceOfBusinessCodeLen: i32 = 3;
pub const ProfileProductCodeLen: i32 = 12;
pub const ProfileSerialNumberLen: i32 = 12;
pub const ProfileDateOfManufactureLen: i32 = 4;

pub const ProfileFaultEncountered: u8 = 0x41;
pub const ProfileNoFaultEncountered: u8 = 0x42;
pub const ProfileManufacturerUnknown: u32 = ObjectManufacturerUnknown;

pub struct Profile {}

impl Profile {
    pub fn new() -> Object {
        Object::new()
    }
}
