// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::object::Object;
use crate::uecho::property::*;

pub const ObjectOperatingStatus: u8 = 0x80;
pub const ObjectManufacturerCode: u8 = 0x8A;
pub const ObjectAnnoPropertyMap: u8 = 0x9D;
pub const ObjectSetPropertyMap: u8 = 0x9E;
pub const ObjectGetPropertyMap: u8 = 0x9F;

pub const ObjectOperatingStatusOn: u8 = 0x30;
pub const ObjectOperatingStatusOff: u8 = 0x31;
pub const ObjectOperatingStatusSize: i32 = 1;
pub const ObjectManufacturerEvaluationCodeMin: u32 = 0xFFFFF0;
pub const ObjectManufacturerEvaluationCodeMax: u32 = 0xFFFFFF;
pub const ObjectManufacturerCodeSize: usize = 3;
pub const ObjectPropertyMapMaxSize: i32 = 16;
pub const ObjectAnnoPropertyMapMaxSize: i32 = (ObjectPropertyMapMaxSize + 1);
pub const ObjectSetPropertyMapMaxSize: i32 = (ObjectPropertyMapMaxSize + 1);
pub const ObjectGetPropertyMapMaxSize: i32 = (ObjectPropertyMapMaxSize + 1);

pub const ObjectManufacturerUnknown: u32 = ObjectManufacturerEvaluationCodeMin;

impl Object {
    pub fn add_mandatory_properties(obj: &mut Object) {
        obj.set_manufacturer_code(ObjectManufacturerUnknown);
    }
}

impl Object {
    pub fn set_manufacturer_code(&mut self, code: u32) -> bool {
        self.set_property_integer(ObjectManufacturerCode, code, ObjectManufacturerCodeSize)
    }

    pub fn manufacturer_code(&mut self) -> &mut Property {
        self.property(ObjectManufacturerCode).unwrap()
    }
}
