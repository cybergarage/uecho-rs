// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::{IpAddr, Ipv4Addr};

use crate::uecho::protocol::message::Message;

pub struct RemoteNode {
    addr: IpAddr,
}

impl RemoteNode {
    pub fn new() -> RemoteNode {
        RemoteNode {
            addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        }
    }

    pub fn from_message(msg: &Message) -> RemoteNode {
        RemoteNode { addr: msg.addr() }
    }

    pub fn set_addr(&mut self, addr: IpAddr) {
        self.addr = addr
    }

    pub fn addr(&self) -> IpAddr {
        self.addr
    }
}

impl PartialEq for RemoteNode {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
