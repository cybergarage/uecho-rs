// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use log::warn;
use nix::sys::socket::sockopt::{IpAddMembership, ReuseAddr, ReusePort};
use nix::sys::socket::{bind, recvfrom, sendto, setsockopt, shutdown, socket};
use nix::sys::socket::{
    AddressFamily, IpMembershipRequest, MsgFlags, Shutdown, SockFlag, SockType, SockaddrLike,
};
use nix::Result;
use socket2::{Domain, Socket, Type};
use std::io;
use std::net::Ipv4Addr;
use std::os::unix::io::RawFd;
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
    thread::sleep(Duration::from_secs(1));
}

pub struct UdpSocket {
    sock: RawFd,
}

impl UdpSocket {
    pub fn new() -> UdpSocket {
        let sock = socket(
            AddressFamily::Inet,
            SockType::Datagram,
            SockFlag::empty(),
            None,
        )
        .unwrap();

        if setsockopt(sock, ReuseAddr, &true).is_err() {
            warn!("SO_REUSEADDR is not supported");
        }
        if setsockopt(sock, ReusePort, &true).is_err() {
            warn!("SO_REUSEPORT is not supported");
        }

        UdpSocket { sock: sock }
    }

    pub fn close(&self) -> Result<()> {
        let res = shutdown(self.sock, Shutdown::Both);
        udp_socket_closewait();
        res
    }

    pub fn bind(&self, addr: &dyn SockaddrLike) -> Result<()> {
        bind(self.sock, addr)
    }

    pub fn send_to(&self, buf: &[u8], addr: &dyn SockaddrLike) -> Result<usize> {
        let flags = MsgFlags::empty();
        sendto(self.sock, buf, addr, flags)
    }

    pub fn recvfrom<T: SockaddrLike>(&self, buf: &mut [u8]) -> Result<(usize, Option<T>)> {
        recvfrom(self.sock, buf)
    }

    pub fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> Result<()> {
        let opt = IpMembershipRequest::new(multiaddr, Some(interface));
        setsockopt(self.sock, IpAddMembership, &opt)
    }
}
