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

use crate::device::*;
use crate::object::Object;
use crate::property::*;

pub const SUPER_OBJECT_CODE: u32 = 0x000000;
pub const OBJECT_OPERATING_STATUS: u8 = 0x80;
pub const OBJECT_FAULT_STATUS: u8 = 0x88;
pub const OBJECT_MANUFACTURER_CODE: u8 = 0x8A;
pub const OBJECT_ANNO_PROPERTY_MAP: u8 = 0x9D;
pub const OBJECT_SET_PROPERTY_MAP: u8 = 0x9E;
pub const OBJECT_GET_PROPERTY_MAP: u8 = 0x9F;

pub const OBJECT_OPERATING_STATUS_ON: u8 = 0x30;
pub const OBJECT_OPERATING_STATUS_OFF: u8 = 0x31;
pub const OBJECT_FAULT_STATUS_ON: u8 = 0x41;
pub const OBJECT_FAULT_STATUS_OFF: u8 = 0x42;
pub const OBJECT_OPERATING_STATUS_SIZE: usize = 1;
pub const OBJECT_MANUFACTURER_EXPERIMENT_CODE_MIN: u32 = 0xFFFFF0;
pub const OBJECT_MANUFACTURER_EXPERIMENT_CODE_MAX: u32 = 0xFFFFFF;
pub const OBJECT_MANUFACTURER_CODE_SIZE: usize = 3;
pub const OBJECT_PROPERTY_MAP_FORMAT1_MAX_SIZE: usize = 15;
pub const OBJECT_ANNO_PROPERTY_MAP_MAX_SIZE: usize = OBJECT_PROPERTY_MAP_FORMAT1_MAX_SIZE + 1;
pub const OBJECT_SET_PROPERTY_MAP_MAX_SIZE: usize = OBJECT_PROPERTY_MAP_FORMAT1_MAX_SIZE + 1;
pub const OBJECT_GET_PROPERTY_MAP_MAX_SIZE: usize = OBJECT_PROPERTY_MAP_FORMAT1_MAX_SIZE + 1;
pub const OBJECT_PROPERTY_MAP_FORMAT2_SIZE: usize = 16;

pub const OBJECT_MANUFACTURER_EXPERIMENT: u32 = 0xFFFFFF;

impl Object {
    pub fn set_manufacturer_code(&mut self, code: u32) -> bool {
        self.set_property_int(
            OBJECT_MANUFACTURER_CODE,
            code,
            OBJECT_MANUFACTURER_CODE_SIZE,
        )
    }

    pub fn manufacturer_code(&mut self) -> &mut Property {
        self.find_property_mut(OBJECT_MANUFACTURER_CODE).unwrap()
    }

    pub fn set_operating_status(&mut self, status: bool) -> bool {
        let status_byte: u8 = if status {
            OBJECT_OPERATING_STATUS_ON
        } else {
            OBJECT_OPERATING_STATUS_OFF
        };
        self.set_property_byte(DEVICE_OPERATING_STATUS, status_byte)
    }

    pub fn operating_status(&mut self) -> Option<u8> {
        self.property_data_as_byte(DEVICE_OPERATING_STATUS)
    }
}
