// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use socket2::{Domain, Socket, Type};
use std::io;
use std::thread;
use std::time::Duration;

pub fn udp_socket_create() -> io::Result<Socket> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None);
    match socket {
        Ok(ref sock) => {
            sock.set_reuse_address(true);
            // sock.set_only_v6(false);
            // sock.set_reuse_port(true);
        }
        Err(ref _err) => {}
    }
    socket
}

pub fn udp_socket_closewait() {
    thread::sleep(Duration::from_secs(2));
}
