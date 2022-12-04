// Copyright (C) 2022 Satoshi Konno All rights reserved.
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

use std::net::{IpAddr, SocketAddr};

use crate::protocol::Message;
use crate::transport::multicast_manager::MulticastManager;
use crate::transport::observer::ObserverObject;
use crate::transport::unicast_manager::UnicastManager;

/// Manager handles all messaging packet between ECHONET-Lite nodes.
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

    pub fn add_observer(&mut self, observer: ObserverObject) -> bool {
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

    pub fn has_interface(&self, addr: IpAddr) -> bool {
        if self.mcast_mgr.has_interface(addr) {
            return true;
        }
        self.ucast_mgr.has_interface(addr)
    }

    pub fn is_running(&self) -> bool {
        if !self.mcast_mgr.is_running() {
            return false;
        }
        if !self.ucast_mgr.is_running() {
            return false;
        }
        true
    }

    pub fn start(&mut self) -> bool {
        if self.is_running() {
            return true;
        }
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

impl Drop for Manager {
    fn drop(&mut self) {
        self.stop();
    }
}
