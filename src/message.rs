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

use crate::node_profile::*;
use crate::object::ObjectCode;
use crate::protocol::Message;
use crate::protocol::Property;
use crate::protocol::ESV;
use crate::util::Bytes;

pub struct SearchMessage {}

impl SearchMessage {
    pub fn from_object_code(code: ObjectCode) -> Message {
        let mut msg = Message::new();

        msg.set_esv(ESV::ReadRequest);
        msg.set_seoj(NODE_PROFILE_OBJECT_CODE);
        msg.set_deoj(code);

        let mut prop = Property::new();
        prop.set_code(NODE_PROFILE_CLASS_SELF_NODE_INSTANCE_LIST_S);
        msg.add_property(prop);

        msg
    }

    pub fn new() -> Message {
        SearchMessage::from_object_code(NODE_PROFILE_OBJECT_CODE)
    }
}

pub struct ResponseErrorMessage {}

impl ResponseErrorMessage {
    pub fn from(req_msg: &Message) -> Message {
        let mut msg = Message::new();
        match req_msg.esv() {
            // 4.2.3.1 Property value write service (no response required) [0x60, 0x50]
            ESV::WriteRequest => {
                msg.set_esv(ESV::WriteRequestError);
            }
            // 4.2.3.2 Property value write service (response required) [0x61,0x71,0x51]
            ESV::WriteRequestResponseRequired => {
                msg.set_esv(ESV::WriteRequestResponseRequiredError);
            }
            // 4.2.3.3 Property value read service [0x62,0x72,0x52]
            ESV::ReadRequest => {
                msg.set_esv(ESV::ReadRequestError);
            }
            // 4.2.3.4 Property value write & read service [0x6E,0x7E,0x5E]
            ESV::WriteReadRequest => {
                msg.set_esv(ESV::WriteReadRequestError);
            }
            // 4.2.3.5 Property value notification service [0x63,0x73,0x53]
            ESV::NotificationRequest => {
                msg.set_esv(ESV::NotificationRequestError);
            }
            // 4.2.3.6 Property value notification service (response required) [0x74, 0x7A]
            ESV::NotificationResponseRequired => {
                msg.set_esv(ESV::NotificationRequestError);
            }
            _ => {
                msg.set_esv(ESV::Unknown);
            }
        }
        msg.set_tid(req_msg.tid());
        msg.set_seoj(req_msg.deoj());
        msg.set_deoj(req_msg.seoj());
        match msg.esv() {
            // 4.2.3.4 Property value write & read service [0x6E,0x7E,0x5E]
            ESV::WriteReadRequestError => {
                for req_prop in req_msg.properties_set().iter() {
                    msg.add_property(req_prop.clone());
                }
                for req_prop in req_msg.properties_set().iter() {
                    msg.add_property(req_prop.clone());
                }
            }
            _ => {
                for req_prop in req_msg.properties().iter() {
                    msg.add_property(req_prop.clone());
                }
            }
        }
        msg
    }
}

pub struct ResponseMessage {}

impl ResponseMessage {
    pub fn from(req_msg: &Message) -> Message {
        let mut msg = Message::new();
        msg.set_tid(req_msg.tid());
        match req_msg.esv() {
            // 4.2.3.2 Property value write service (response required) [0x61,0x71,0x51]
            ESV::WriteRequestResponseRequired => {
                msg.set_esv(ESV::WriteResponse);
            }
            // 4.2.3.3 Property value read service [0x62,0x72,0x52]
            ESV::ReadRequest => {
                msg.set_esv(ESV::ReadResponse);
            }
            // 4.2.3.4 Property value write & read service [0x6E,0x7E,0x5E]
            ESV::WriteReadRequest => {
                msg.set_esv(ESV::WriteReadResponse);
            }
            // 4.2.3.5 Property value notification service [0x63,0x73,0x53]
            ESV::NotificationRequest => {
                msg.set_esv(ESV::Notification);
            }
            // 4.2.3.6 Property value notification service (response required) [0x74, 0x7A]
            ESV::NotificationResponseRequired => {
                msg.set_esv(ESV::NotificationResponse);
            }
            _ => {
                msg.set_esv(ESV::Unknown);
            }
        }
        msg.set_seoj(req_msg.deoj());
        msg.set_deoj(req_msg.seoj());
        msg
    }
}

pub struct NodeProfileMessage<'a> {
    obj_codes: Vec<ObjectCode>,
    msg: &'a Message,
}

impl NodeProfileMessage<'_> {
    pub fn from_message<'a>(msg: &'a Message) -> NodeProfileMessage<'a> {
        NodeProfileMessage {
            msg: msg,
            obj_codes: Vec::new(),
        }
    }

    pub fn object_codes(&self) -> &Vec<ObjectCode> {
        return &self.obj_codes;
    }

    pub fn parse(&mut self) -> bool {
        let opc = self.msg.opc();
        for n in 0..opc {
            let prop = self.msg.property(n);
            if prop.code() != NODE_PROFILE_CLASS_SELF_NODE_INSTANCE_LIST_S {
                continue;
            }
            let prop_data = prop.data();
            let prop_data_len = prop_data.len();
            if prop_data_len < 1 {
                continue;
            }
            let num_objs = prop_data[0] as usize;
            if prop_data_len < ((num_objs * 3) + 1) {
                continue;
            }
            for idx in (1..prop_data_len).step_by(3) {
                if (prop_data.len() - idx) < 3 {
                    continue;
                }
                let obj_code_bytes = &prop_data[idx..(idx + 3)];
                let obj_code = Bytes::to_u32(obj_code_bytes) as ObjectCode;
                self.obj_codes.push(obj_code);
            }
        }
        true
    }
}

impl Message {
    pub fn is_node_profile_message(&self) -> bool {
        let esv = self.esv();
        if esv != ESV::Notification && esv != ESV::ReadResponse {
            return false;
        }
        let dst_obj = self.deoj();
        if dst_obj != NODE_PROFILE_OBJECT_CODE && dst_obj != NODE_PROFILE_OBJECT_READ_ONLY {
            return false;
        }
        true
    }
}
