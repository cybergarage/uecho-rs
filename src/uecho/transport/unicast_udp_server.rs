// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::UdpSocket;
use std::ptr;

pub struct UnicastUdpServer {}

impl UnicastUdpServer {
    pub fn new() -> UnicastUdpServer {
        UnicastUdpServer {}
    }

    pub fn start(&self) -> bool {
        /*
        let mut socket = UdpSocket::bind("127.0.0.1:34254");
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;
        */
        true
    }

    pub fn stop(&self) -> bool {
        true
    }
}
