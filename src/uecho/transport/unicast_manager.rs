// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::transport::unicast_tcp_server::UnicastTcpServer;
use crate::uecho::transport::unicast_udp_server::UnicastUdpServer;

pub struct UnicastManager {
    udp_server: UnicastUdpServer,
    tcp_server: UnicastTcpServer,
}

impl UnicastManager {
    pub fn new() -> UnicastManager {
        UnicastManager {
            udp_server: UnicastUdpServer::new(),
            tcp_server: UnicastTcpServer::new(),
        }
    }

    pub fn start(&self) -> bool {
        if !self.udp_server.start() {
            self.stop();
            return false;
        }
        if !self.tcp_server.start() {
            self.stop();
            return false;
        }
        true
    }

    pub fn stop(&self) -> bool {
        let mut ret = self.udp_server.stop();
        ret |= self.tcp_server.stop();
        ret
    }
}
