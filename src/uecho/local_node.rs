// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::io;
use std::net::SocketAddr;

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::manager::*;
use crate::uecho::transport::observer::*;

pub struct LocalNode {
    transport_mgr: Manager,
}

impl LocalNode {
    pub fn new() -> LocalNode {
        LocalNode {
            transport_mgr: Manager::new(),
        }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.transport_mgr.local_addr()
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        self.transport_mgr.add_observer(observer.clone())
    }

    pub fn send_messagee(&self, to_addr: SocketAddr, msg: &Message) -> bool {
        self.transport_mgr.send_message(to_addr, msg)
    }

    pub fn notify(&self, msg: &Message) -> bool {
        self.transport_mgr.notify(msg)
    }

    pub fn start(&mut self) -> bool {
        if !self.transport_mgr.start() {
            return false;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        if !self.transport_mgr.stop() {
            return false;
        }
        true
    }
}
