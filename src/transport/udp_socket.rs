// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use log::warn;
use socket2::{Domain, Socket, Type};
use std::io;
use std::thread;
use std::time::Duration;

pub fn udp_socket_create() -> io::Result<Socket> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None);
    match socket {
        Ok(ref sock) => {
            if sock.set_reuse_address(true).is_err() {
                warn!("SO_REUSEADDR is not supported");
            }
            // NOTE: set_reuse_port() is not supported yet in socket2 v0.4.5.
            // if sock.set_reuse_port(true).is_err() {
            //     warn!("SO_REUSEPORT is not supported");
            // }
        }
        Err(ref _err) => {}
    }
    socket
}

pub fn udp_socket_closewait() {
    thread::sleep(Duration::from_secs(2));
}
