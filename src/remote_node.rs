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

use std::net::{IpAddr, Ipv4Addr};

use crate::message::NodeProfileMessage;
use crate::object::*;
use crate::protocol::message::Message;

pub struct RemoteNode {
    addr: IpAddr,
    objects: Vec<Object>,
}

impl RemoteNode {
    pub fn new() -> RemoteNode {
        RemoteNode {
            addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            objects: Vec::new(),
        }
    }

    pub fn from_message(msg: &Message) -> RemoteNode {
        let mut node = RemoteNode {
            addr: msg.addr(),
            objects: Vec::new(),
        };
        node.parse(msg);
        node
    }

    pub fn addr(&self) -> IpAddr {
        self.addr
    }

    pub fn set_addr(&mut self, addr: IpAddr) {
        self.addr = addr
    }

    pub fn add_object(&mut self, obj: Object) -> bool {
        self.objects.push(obj);
        true
    }

    pub fn objects(&self) -> &Vec<Object> {
        return &self.objects;
    }

    pub fn find_object(&self, code: ObjectCode) -> Option<&Object> {
        for n in 0..self.objects.len() {
            if self.objects[n].code() == code {
                return Some(&self.objects[n]);
            }
        }
        None
    }

    fn parse(&mut self, msg: &Message) -> bool {
        if !msg.is_node_profile_message() {
            return false;
        }
        let profile_msg = NodeProfileMessage::from_message(msg);
        if !profile_msg.parse() {
            return false;
        }
        true
    }
}

impl Clone for RemoteNode {
    fn clone(&self) -> RemoteNode {
        let mut node = RemoteNode {
            addr: self.addr().clone(),
            objects: Vec::new(),
        };
        for obj in self.objects() {
            node.add_object(obj.clone());
        }
        node
    }
}

impl<'a> PartialEq for RemoteNode {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
