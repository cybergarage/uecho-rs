// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::transport::server::Server;
use std::net::UdpSocket;
use std::ptr;

pub struct UnicastServer {
    //sock: &'a UdpSocket,
}

impl UnicastServer {
    pub fn new() -> UnicastServer {
        UnicastServer {}
    }
}

impl Server for UnicastServer {
    fn start(&self) -> bool {
        true
    }

    fn stop(&self) -> bool {
        true
    }
}
