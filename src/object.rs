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

use std::cmp::PartialEq;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::database::StandardDatabase;
use crate::device::*;
use crate::message::{ResponseErrorMessage, ResponseMessage};
use crate::property::{Property, PropertyCode, PropertyData};
use crate::protocol::{Message, ESV};
use crate::super_object::*;
use crate::util::Bytes;
use crate::util::OID;

/// ObjectCode represents an ECHONET-Lite object code.
pub type ObjectCode = u32;

/// Each ECHONET-Lite node has objects. Object represents an ECHONET-Lite Object in an ECHONET-Lite node.
pub struct Object {
    codes: [u8; 3],
    name: String,
    class_name: String,
    properties: HashMap<PropertyCode, Property>,
}

impl Object {
    pub fn new() -> Object {
        Object {
            codes: [0, 0, 0],
            name: String::from(""),
            class_name: String::from(""),
            properties: HashMap::new(),
        }
    }

    pub fn from_code(code: ObjectCode) -> Object {
        let mut obj = Object::new();
        obj.set_code(code);
        obj
    }

    pub fn set_code(&mut self, code: ObjectCode) -> &mut Self {
        self.codes[0] = ((code & 0xFF0000) >> 16) as u8;
        self.codes[1] = ((code & 0x00FF00) >> 8) as u8;
        self.codes[2] = (code & 0x0000FF) as u8;
        self
    }

    pub fn set_class_group_code(&mut self, code: u8) -> &mut Self {
        self.codes[0] = code;
        self
    }

    pub fn set_class_code(&mut self, code: u8) -> &mut Self {
        self.codes[1] = code;
        self
    }

    pub fn set_instance_code(&mut self, code: u8) -> &mut Self {
        self.codes[2] = code;
        self
    }

    pub fn code(&self) -> ObjectCode {
        let mut code = 0 as ObjectCode;
        code |= ((self.codes[0] as ObjectCode) << 16) & 0xFF0000;
        code |= ((self.codes[1] as ObjectCode) << 8) & 0x00FF00;
        code |= (self.codes[2] as ObjectCode) & 0x0000FF;
        code
    }

    pub fn class_group_code(&self) -> u8 {
        self.codes[0]
    }

    pub fn class_code(&self) -> u8 {
        self.codes[1]
    }

    pub fn instance_code(&self) -> u8 {
        self.codes[2]
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_class_name(&mut self, name: String) -> &mut Self {
        self.class_name = name;
        self
    }

    pub fn class_name(&self) -> &String {
        &self.class_name
    }

    pub fn add_property(&mut self, prop: Property) -> bool {
        let code = prop.code();
        self.properties.entry(code).or_insert(prop);
        self.update_property_maps();
        true
    }

    pub fn properties(&self) -> Values<'_, PropertyCode, Property> {
        self.properties.values()
    }

    pub fn find_property_mut(&mut self, code: PropertyCode) -> Option<&mut Property> {
        self.properties.get_mut(&code)
    }

    pub fn find_property(&self, code: PropertyCode) -> Option<&Property> {
        self.properties.get(&code)
    }

    pub fn has_property(&self, code: PropertyCode) -> bool {
        if self.properties.get(&code).is_none() {
            return false;
        }
        true
    }

    pub fn set_property_data(&mut self, code: PropertyCode, data: &[u8]) -> bool {
        match self.find_property_mut(code).as_mut() {
            Some(prop) => {
                prop.set_data(data);
                true
            }
            None => false,
        }
    }

    pub fn set_property_byte(&mut self, code: PropertyCode, v: u8) -> bool {
        let data: &[u8] = &[v];
        self.set_property_data(code, data)
    }

    pub fn set_property_bytes(&mut self, code: PropertyCode, data: &[u8]) -> bool {
        self.set_property_data(code, data)
    }

    pub fn set_property_int(&mut self, code: PropertyCode, val: u32, byte_size: usize) -> bool {
        let mut buf = vec![0; byte_size];
        Bytes::from_u32(val, &mut buf);
        self.set_property_data(code, &buf)
    }

    pub fn property_data(&self, code: PropertyCode) -> Option<&PropertyData> {
        match self.find_property(code) {
            Some(prop) => return Some(prop.data()),
            None => return None,
        }
    }

    pub fn property_data_as_bytes(&self, code: PropertyCode) -> Option<&[u8]> {
        match self.find_property(code) {
            Some(prop) => return Some(prop.data_as_bytes()),
            None => return None,
        }
    }

    pub fn property_data_as_byte(&self, code: PropertyCode) -> Option<u8> {
        match self.find_property(code) {
            Some(prop) => return Some(prop.data_as_byte()),
            None => return None,
        }
    }

    pub fn property_data_as_int(&self, code: PropertyCode) -> Option<u32> {
        match self.find_property(code) {
            Some(prop) => return Some(prop.data_as_int()),
            None => return None,
        }
    }

    pub fn equals_property_data(&mut self, code: PropertyCode, data: &[u8]) -> bool {
        match self.find_property_mut(code) {
            Some(prop) => prop.equals_data(data),
            None => false,
        }
    }

    pub fn add_standard_properties(&mut self, code: ObjectCode) -> bool {
        let db = StandardDatabase::shared();
        let std_obj = db.find_object(code & 0xFFFF00);
        match std_obj {
            Some(obj) => {
                self.set_class_name(obj.class_name().clone());
                for (prop_code, std_prop) in &obj.properties {
                    self.add_property(std_prop.clone());
                    // Sets default standard property data
                    match *prop_code {
                        OBJECT_MANUFACTURER_CODE => {
                            self.set_manufacturer_code(OBJECT_MANUFACTURER_EXPERIMENT);
                        }
                        DEVICE_OPERATING_STATUS => {
                            self.set_operating_status(false);
                        }
                        DEVICE_INSTALLATION_LOCATION => {
                            self.set_installation_location(DEVICE_INSTALLATION_LOCATION_UNKNOWN);
                        }
                        DEVICE_FAULT_STATUS => {
                            self.set_fault_status(false);
                        }
                        DEVICE_STANDARD_VERSION => {
                            self.set_standard_version(DEVICE_DEFAULT_VERSION_APPENDIX);
                        }
                        DEVICE_IDENTIFICATION_NUMBER => {
                            self.set_id(OID::new(OBJECT_MANUFACTURER_EXPERIMENT).bytes());
                        }
                        _ => {}
                    }
                }
                true
            }
            None => false,
        }
    }

    fn update_property_maps(&mut self) {
        // Annex 1 Property Map Description Format
        fn to_property_map_bytes<'a>(maps: &'a Vec<u8>, map_bytes: &'a mut Vec<u8>) -> &'a [u8] {
            map_bytes.clear();
            map_bytes.push(maps.len() as u8);
            if maps.len() <= OBJECT_PROPERTY_MAP_FORMAT1_MAX_SIZE {
                for prop_code in maps.iter() {
                    map_bytes.push(*prop_code);
                }
            } else {
                for _n in 0..OBJECT_PROPERTY_MAP_FORMAT2_SIZE {
                    map_bytes.push(0x00 as u8);
                }
                for prop_code in maps.iter() {
                    // 1 <= prop_map_row <= 16
                    let prop_map_row = ((*prop_code - 0x80) & 0x0F) + 1;
                    // 0 <= prop_map_bit <= 7
                    let prop_map_bit = (((*prop_code - 0x80) & 0xF0) >> 4) & 0x0F;
                    map_bytes[prop_map_row as usize] |= (0x01 << prop_map_bit) & 0x0F;
                }
            }
            map_bytes
        }

        let mut get_map = Vec::new();
        let mut set_map = Vec::new();
        let mut anno_map = Vec::new();
        for prop in self.properties.values() {
            if prop.is_readable() {
                get_map.push(prop.code());
            }
            if prop.is_writable() {
                set_map.push(prop.code());
            }
            if prop.is_announceable() {
                anno_map.push(prop.code());
            }
        }

        let mut map_bytes: Vec<u8> = Vec::new();
        self.set_property_data(
            OBJECT_GET_PROPERTY_MAP,
            to_property_map_bytes(&get_map, &mut map_bytes),
        );
        self.set_property_data(
            OBJECT_SET_PROPERTY_MAP,
            to_property_map_bytes(&set_map, &mut map_bytes),
        );
        self.set_property_data(
            OBJECT_ANNO_PROPERTY_MAP,
            to_property_map_bytes(&anno_map, &mut map_bytes),
        );
    }

    pub fn message_received(&self, req_msg: &Message) -> Option<Message> {
        let mut res_msg = ResponseMessage::from(req_msg);
        match req_msg.esv() {
            // 4.2.3.2 Property value write service (response required) [0x61,0x71,0x51]
            // 4.2.3.6 Property value notification service (response required) [0x74, 0x7A]
            ESV::WriteRequestResponseRequired | ESV::NotificationResponseRequired => {
                for req_prop in req_msg.properties() {
                    let mut res_prop = crate::protocol::Property::new();
                    res_prop.set_code(req_prop.code());
                    res_msg.add_property(res_prop);
                }
            }
            // 4.2.3.3 Property value read service [0x62,0x72,0x52]
            // 4.2.3.5 Property value notification service [0x63,0x73,0x53]
            ESV::ReadRequest | ESV::NotificationRequest => {
                for req_prop in req_msg.properties() {
                    let obj_prop = self.find_property(req_prop.code());
                    if obj_prop.is_none() {
                        return Some(ResponseErrorMessage::from(req_msg));
                    }
                    let obj_prop = obj_prop.unwrap();
                    res_msg.add_property(crate::protocol::Property::from(
                        obj_prop.code(),
                        obj_prop.data().clone(),
                    ));
                }
            }
            // 4.2.3.4 Property value write & read service [0x6E,0x7E,0x5E]
            ESV::WriteReadRequest => {
                for req_prop in req_msg.properties_set() {
                    let mut res_prop = crate::protocol::Property::new();
                    res_prop.set_code(req_prop.code());
                    res_msg.add_property_set(res_prop);
                }
                for req_prop in req_msg.properties_get() {
                    let obj_prop = self.find_property(req_prop.code());
                    if obj_prop.is_none() {
                        return Some(ResponseErrorMessage::from(req_msg));
                    }
                    let obj_prop = obj_prop.unwrap();
                    res_msg.add_property_get(crate::protocol::Property::from(
                        obj_prop.code(),
                        obj_prop.data().clone(),
                    ));
                }
            }
            _ => {
                return None;
            }
        }
        Some(res_msg)
    }
}

impl Clone for Object {
    fn clone(&self) -> Object {
        let mut obj = Object {
            codes: [self.codes[0], self.codes[1], self.codes[2]],
            name: self.name().clone(),
            class_name: self.class_name().clone(),
            properties: HashMap::new(),
        };
        for prop in self.properties() {
            obj.add_property(prop.clone());
        }
        obj
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        if self.code() != other.code() {
            return false;
        }
        true
    }
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code().hash(state);
    }
}
