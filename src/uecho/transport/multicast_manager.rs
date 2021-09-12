// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::interface::*;
use crate::uecho::transport::multicast_server::MulticastServer;
use crate::uecho::transport::observer::*;

pub struct MulticastManager {
    mcast_servers: Vec<MulticastServer>,
}

impl MulticastManager {
    pub fn new() -> MulticastManager {
        MulticastManager {
            mcast_servers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        for mcast_server in self.mcast_servers.iter_mut() {
            if mcast_server.add_observer(observer.clone()) {
                return false;
            }
        }
        true
    }

    pub fn notify(&self, msg: &Message) -> bool {
        for mcast_server in self.mcast_servers.iter() {
            if mcast_server.notify(msg) {
                return true;
            }
        }
        false
    }

    pub fn start(&mut self) -> bool {
        for ifaddr in get_v4_interfaces() {
            let mut mcast_server = MulticastServer::new();
            if !mcast_server.bind(ifaddr) {
                self.stop();
                return false;
            }
            if !mcast_server.start() {
                self.stop();
                return false;
            }
            self.mcast_servers.push(mcast_server);
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        let mut is_all_server_stopped = true;
        for mcast_server in self.mcast_servers.iter_mut() {
            if !&mcast_server.stop() {
                is_all_server_stopped = false;
            }
        }
        self.mcast_servers.clear();
        is_all_server_stopped
    }
}
