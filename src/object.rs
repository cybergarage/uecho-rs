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
use crate::message::{ResponseErrorMessage, ResponseMessage};
use crate::property::{Property, PropertyCode, PropertyData};
use crate::protocol::{Esv, Message};
use crate::super_object::*;
use crate::util::Bytes;

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
        self.properties.insert(code, prop);
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

    pub fn property_data(&mut self, code: PropertyCode) -> Option<&PropertyData> {
        match self.find_property(code) {
            Some(prop) => return Some(prop.data()),
            None => return None,
        }
    }

    pub fn property_data_as_bytes(&mut self, code: PropertyCode) -> Option<&[u8]> {
        match self.find_property(code) {
            Some(prop) => return Some(prop.data_as_bytes()),
            None => return None,
        }
    }

    pub fn add_standard_properties(&mut self, code: ObjectCode) -> bool {
        let db = StandardDatabase::shared();
        let std_obj = db.find_object(code & 0xFFFF00);
        match std_obj {
            Some(obj) => {
                self.set_class_name(obj.class_name().clone());
                for (prop_code, std_prop) in &obj.properties {
                    let mut prop = std_prop.clone();
                    // Sets default standard property data
                    match *prop_code {
                        OBJECT_MANUFACTURER_CODE => {
                            prop.set_int_data(
                                OBJECT_MANUFACTURER_EXPERIMENT,
                                OBJECT_MANUFACTURER_CODE_SIZE,
                            );
                        }
                        _ => {}
                    }
                    self.add_property(prop);
                }
                true
            }
            None => false,
        }
    }

    pub fn equals_property_data(&mut self, code: PropertyCode, data: &[u8]) -> bool {
        match self.find_property_mut(code) {
            Some(prop) => prop.equals_data(data),
            None => false,
        }
    }

    pub fn message_received(&self, req_msg: &Message) -> Option<Message> {
        let mut res_msg = ResponseMessage::from(req_msg);
        match req_msg.esv() {
            // 4.2.3.2 Property value write service (response required) [0x61,0x71,0x51]
            // 4.2.3.6 Property value notification service (response required) [0x74, 0x7A]
            Esv::WriteRequestResponseRequired | Esv::NotificationResponseRequired => {
                for req_prop in req_msg.properties() {
                    let mut res_prop = crate::protocol::Property::new();
                    res_prop.set_code(req_prop.code());
                    res_msg.add_property(res_prop);
                }
            }
            // 4.2.3.3 Property value read service [0x62,0x72,0x52]
            // 4.2.3.5 Property value notification service [0x63,0x73,0x53]
            Esv::ReadRequest | Esv::NotificationRequest => {
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
            Esv::WriteReadRequest => {
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
