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

use crate::object::Object;
use crate::property::*;

pub const SUPER_OBJECT_CODE: u32 = 0x000000;
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
        self.find_property_mut(OBJECT_MANUFACTURER_CODE).unwrap()
    }
}
