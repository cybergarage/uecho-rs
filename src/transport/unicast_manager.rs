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
use crate::transport::interface::*;
use crate::transport::observer::ObserverObject;
use crate::transport::unicast_server::UnicastServer;

pub struct UnicastManager {
    udp_servers: Vec<UnicastServer>,
}

impl UnicastManager {
    pub fn new() -> UnicastManager {
        UnicastManager {
            udp_servers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: ObserverObject) -> bool {
        for udp_server in self.udp_servers.iter_mut() {
            if !udp_server.add_observer(observer.clone()) {
                return false;
            }
        }
        true
    }

    pub fn send(&self, to_addr: SocketAddr, msg: &Message) -> bool {
        for udp_server in self.udp_servers.iter() {
            if udp_server.send(to_addr, msg) {
                return true;
            }
        }
        false
    }

    pub fn is_running(&self) -> bool {
        if self.udp_servers.len() == 0 {
            return false;
        }
        true
    }

    pub fn has_interface(&self, addr: IpAddr) -> bool {
        for udp_server in self.udp_servers.iter() {
            if udp_server.ifaddr().is_err() {
                continue;
            }
            if udp_server.ifaddr().unwrap().ip() == addr {
                return true;
            }
        }
        false
    }

    pub fn start(&mut self) -> bool {
        if self.is_running() {
            return true;
        }

        for ifaddr in get_all_interfaces() {
            let mut udp_server = UnicastServer::new();
            if !udp_server.bind(ifaddr) {
                self.stop();
                return false;
            }
            if !udp_server.start() {
                self.stop();
                return false;
            }
            self.udp_servers.push(udp_server);
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        let mut is_all_server_stopped = true;
        for udp_server in self.udp_servers.iter_mut() {
            if !&udp_server.stop() {
                is_all_server_stopped = false;
            }
        }
        self.udp_servers.clear();
        is_all_server_stopped
    }
}

impl Drop for UnicastManager {
    fn drop(&mut self) {
        self.stop();
    }
}
