// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::transport::server::Server;

pub struct Manager {
    servers: Vec<Box<Server>>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            servers: Vec::new(),
        }
    }

    pub fn start(&self) -> bool {
        for n in 0..self.servers.len() {
            if !self.servers[n].start() {
                return false;
            }
        }
        true
    }

    pub fn stop(&self) -> bool {
        for n in 0..self.servers.len() {
            if !self.servers[n].stop() {
                return false;
            }
        }
        true
    }
}
