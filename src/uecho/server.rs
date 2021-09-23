// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::io;
use std::net::SocketAddr;

use crate::uecho::transport::manager::*;

pub struct Server {
    transport_mgr: Manager,
}

impl Server {
    pub fn new() -> Server {
        Server {
            transport_mgr: Manager::new(),
        }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.transport_mgr.local_addr()
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
