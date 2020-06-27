// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::multicast_manager::MulticastManager;
use crate::uecho::transport::unicast_manager::UnicastManager;
use std::net::ToSocketAddrs;

pub struct Manager {
    ucast_mgr: UnicastManager,
    mcast_mgr: MulticastManager,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            ucast_mgr: UnicastManager::new(),
            mcast_mgr: MulticastManager::new(),
        }
    }

    pub fn send_messagee<A: ToSocketAddrs>(&self, msg: &Message, addr: A) -> bool {
        self.ucast_mgr.send_message(msg, addr)
    }

    pub fn start(&mut self) -> bool {
        if !self.ucast_mgr.start() {
            self.stop();
            return false;
        }
        if !self.mcast_mgr.start() {
            self.stop();
            return false;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        let mut ret = self.ucast_mgr.stop();
        ret |= self.mcast_mgr.stop();
        ret
    }
}
