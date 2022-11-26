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

use std::io;
use std::net::SocketAddr;

use crate::protocol::Message;
use crate::transport::multicast_manager::MulticastManager;
use crate::transport::unicast_manager::UnicastManager;
use crate::transport::*;

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
        if !self.mcast_mgr.start() {
            self.stop();
            return false;
        }
        if !self.ucast_mgr.start() {
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
