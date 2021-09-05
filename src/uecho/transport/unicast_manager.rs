// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::SocketAddr;

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::interface::*;
use crate::uecho::transport::observer::*;
use crate::uecho::transport::unicast_udp_server::UnicastUdpServer;

pub struct UnicastManager {
    udp_servers: Vec<UnicastUdpServer>,
}

impl UnicastManager {
    pub fn new() -> UnicastManager {
        UnicastManager {
            udp_servers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        for udp_server in self.udp_servers.iter_mut() {
            if udp_server.add_observer(observer.clone()) {
                return false;
            }
        }
        true
    }

    pub fn send_message(&self, to_addr: SocketAddr, msg: &Message) -> bool {
        for udp_server in self.udp_servers.iter() {
            if udp_server.send_message(to_addr, msg) {
                return true;
            }
        }
        false
    }

    pub fn start(&mut self) -> bool {
        for ifaddr in get_v4_interfaces() {
            let mut udp_server = UnicastUdpServer::new();
            if udp_server.bind(ifaddr) {
                self.stop();
                return false;
            }
            if udp_server.start() {
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
