// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::{IpAddr, Ipv4Addr};

use crate::uecho::node::Node;
use crate::uecho::object::*;
use crate::uecho::protocol::message::Message;

pub struct RemoteNode {
    addr: IpAddr,
    objects: Objects,
}

impl Node for RemoteNode {
    fn objects(&mut self) -> &Objects {
        &self.objects
    }

    fn addr(&self) -> IpAddr {
        self.addr
    }
}

impl RemoteNode {
    pub fn new() -> RemoteNode {
        RemoteNode {
            addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            objects: objects_new(),
        }
    }

    pub fn from_message(msg: &Message) -> RemoteNode {
        RemoteNode {
            addr: msg.addr(),
            objects: objects_new(),
        }
    }

    pub fn set_addr(&mut self, addr: IpAddr) {
        self.addr = addr
    }
}

impl PartialEq for RemoteNode {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
