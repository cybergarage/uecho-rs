// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::transport::server::Server;
use std::net::UdpSocket;
use std::ptr;

pub struct UnicastTcpServer {}

impl UnicastTcpServer {
    pub fn new() -> UnicastTcpServer {
        UnicastTcpServer {}
    }

    pub fn start(&self) -> bool {
        true
    }

    pub fn stop(&self) -> bool {
        true
    }
}
