// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::{IpAddr, Ipv4Addr};

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
        RemoteNode {
            addr: msg.addr(),
            objects: Vec::new(),
        }
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

    pub fn get_object(&self, code: ObjectCode) -> Option<&Object> {
        for n in 0..self.objects.len() {
            if self.objects[n].code() == code {
                return Some(&self.objects[n]);
            }
        }
        None
    }
}

impl<'a> PartialEq for RemoteNode {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
