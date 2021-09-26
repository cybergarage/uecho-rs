// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::io;
use std::net::SocketAddr;

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::multicast_manager::MulticastManager;
use crate::uecho::transport::observer::*;
use crate::uecho::transport::unicast_manager::UnicastManager;

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

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        if !self.ucast_mgr.add_observer(observer.clone()) {
            return false;
        }
        if !self.mcast_mgr.add_observer(observer.clone()) {
            return false;
        }
        true
    }

    pub fn send(&self, to_addr: SocketAddr, msg: &Message) -> bool {
        self.ucast_mgr.send(to_addr, msg)
    }

    pub fn notify(&self, msg: &Message) -> bool {
        self.mcast_mgr.notify(msg)
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.ucast_mgr.local_addr()
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
