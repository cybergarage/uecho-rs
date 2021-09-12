// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use net2::UdpBuilder;
use std::{io, net::ToSocketAddrs};

#[cfg(not(target_os = "windows"))]
use net2::unix::UnixUdpBuilderExt;

#[cfg(not(target_os = "windows"))]
pub fn create_udp_socket<A: ToSocketAddrs>(addr: A) -> io::Result<std::net::UdpSocket> {
    net2::UdpBuilder::new_v4()?
        .reuse_address(true)?
        .reuse_port(true)?
        .bind(addr)
}

#[cfg(target_os = "windows")]
pub fn create_udp_socket<A: ToSocketAddrs>(addr: A) -> io::Result<std::net::UdpSocket> {
    net2::UdpBuilder::new_v4()?
        .reuse_address(true)?
        .bind((addr))
}
