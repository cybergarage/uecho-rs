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

use crate::object::*;
use crate::super_object::*;

pub const NODE_PROFILE_OBJECT_CODE: u32 = 0x0EF001;
pub const NODE_PROFILE_OBJECT_READ_ONLY: u32 = 0x0EF002;
pub const NODE_PROFILE_CLASS_CODE: u8 = 0xF0;
pub const NODE_PROFILE_INSTANCE_GENERAL_CODE: u8 = 0x01;
pub const NODE_PROFILE_INSTANCE_TRANSMISSION_ONLY_CODE: u8 = 0x02;

pub const NODE_PROFILE_CLASS_OPERATING_STATUS: u8 = OBJECT_OPERATING_STATUS;
pub const NODE_PROFILE_CLASS_VERSION_INFORMATION: u8 = 0x82;
pub const NODE_PROFILE_CLASS_IDENTIFICATION_NUMBER: u8 = 0x83;
pub const NODE_PROFILE_CLASS_FAULT_CONTENT: u8 = 0x89;
pub const NODE_PROFILE_CLASS_UNIQUE_IDENTIFIER_DATA: u8 = 0xBF;
pub const NODE_PROFILE_CLASS_NUMBER_OF_SELF_NODE_INSTANCES: u8 = 0xD3;
pub const NODE_PROFILE_CLASS_NUMBER_OF_SELF_NODE_CLASSES: u8 = 0xD4;
pub const NODE_PROFILE_CLASS_INSTANCE_LIST_NOTIFICATION: u8 = 0xD5;
pub const NODE_PROFILE_CLASS_SELF_NODE_INSTANCE_LIST_S: u8 = 0xD6;
pub const NODE_PROFILE_CLASS_SELF_NODE_CLASS_LIST_S: u8 = 0xD7;

pub const NODE_PROFILE_CLASS_OPERATING_STATUS_SIZE: i32 = 1;
pub const NODE_PROFILE_CLASS_VERSION_INFORMATION_SIZE: i32 = 4;
pub const NODE_PROFILE_CLASS_IDENTIFICATION_MANUFACTURER_CODE_SIZE: i32 = 3;
pub const NODE_PROFILE_CLASS_IDENTIFICATION_UNIQUE_ID_SIZE: i32 = 13;
pub const NODE_PROFILE_CLASS_IDENTIFICATION_NUMBER_SIZE: i32 = 1
    + NODE_PROFILE_CLASS_IDENTIFICATION_MANUFACTURER_CODE_SIZE
    + NODE_PROFILE_CLASS_IDENTIFICATION_UNIQUE_ID_SIZE;
pub const NODE_PROFILE_CLASS_FAULT_CONTENT_SIZE: i32 = 2;
pub const NODE_PROFILE_CLASS_UNIQUE_IDENTIFIER_DATA_SIZE: i32 = 2;
pub const NODE_PROFILE_CLASS_NUMBER_OF_SELF_NODE_INSTANCES_SIZE: i32 = 3;
pub const NODE_PROFILE_CLASS_NUMBER_OF_SELF_NODE_CLASSES_SIZE: i32 = 2;
pub const NODE_PROFILE_CLASS_SELF_NODE_INSTANCE_LIST_S_MAX: i32 = 0xFF;
pub const NODE_PROFILE_CLASS_SELF_NODE_CLASS_LIST_S_MAX: i32 = 0xFF;
pub const NODE_PROFILE_CLASS_INSTANCE_LIST_NOTIFICATION_MAX: i32 =
    NODE_PROFILE_CLASS_SELF_NODE_INSTANCE_LIST_S_MAX;

pub const NODE_PROFILE_CLASS_OPERATING_STATUS_ON: u8 = OBJECT_OPERATING_STATUS_ON;
pub const NODE_PROFILE_CLASS_OPERATING_STATUS_OFF: u8 = OBJECT_OPERATING_STATUS_OFF;
pub const NODE_PROFILE_CLASS_BOOTING: u8 = 0x30;
pub const NODE_PROFILE_CLASS_NOT_BOOTING: u8 = 0x31;
pub const LOWER_COMMUNICATION_LAYER_PROTOCOL_TYPE: u8 = 0xFE;

pub struct NodeProfile<'a> {
    obj: &'a Object,
}

impl NodeProfile<'_> {
    pub fn new() -> Object {
        let mut obj = Object::new();
        obj.add_standard_properties(SUPER_OBJECT_CODE);
        obj.add_standard_properties(NODE_PROFILE_OBJECT_CODE);
        obj
    }

    pub fn from(obj: &Object) -> NodeProfile {
        NodeProfile { obj: obj }
    }

    pub fn update(&mut self, objs: &Vec<Object>) {}
}
