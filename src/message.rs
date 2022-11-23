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
use crate::protocol::esv::*;
use crate::protocol::message::Message;
use crate::protocol::property::Property;
use crate::util::bytes::Bytes;

pub struct SearchMessage {}

impl SearchMessage {
    pub fn from_object_code(code: ObjectCode) -> Message {
        let mut msg = Message::new();

        msg.set_esv(Esv::ReadRequest);
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
            if prop_data.len() < 1 {
                continue;
            }
            // let num_objs = prop_data[0] as usize;
            for idx in (1..prop_data.len()).step_by(3) {
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
        if esv != Esv::Notification && esv != Esv::ReadResponse {
            return false;
        }
        let dst_obj = self.deoj();
        if dst_obj != NODE_PROFILE_OBJECT_CODE && dst_obj != NODE_PROFILE_OBJECT_READ_ONLY {
            return false;
        }
        true
    }
}
