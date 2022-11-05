// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use log::warn;
use nix::sys::socket;
use nix::sys::socket::sockopt::{IpAddMembership, ReuseAddr, ReusePort};
use nix::sys::socket::{bind, recvfrom, sendto, setsockopt, shutdown, socket};
use nix::sys::socket::{
    AddressFamily, IpMembershipRequest, MsgFlags, Shutdown, SockFlag, SockType, SockaddrIn,
};
use nix::Result;
use socket2::{Domain, Socket, Type};
use std::io;
use std::net::Ipv4Addr;
use std::net::{SocketAddr, SocketAddrV4};
use std::os::unix::io::RawFd;
use std::str::FromStr;
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
    ifaddr: Option<SocketAddr>,
}

fn stdaddr_to_nixaddr(ifaddr: SocketAddr) -> SockaddrIn {
    let addr = ifaddr.ip();
    let port = ifaddr.port();
    let std_sa = SocketAddrV4::from_str(&format!("{}:{}", addr, port)).unwrap();
    SockaddrIn::from(std_sa)
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

        UdpSocket {
            sock: sock,
            ifaddr: None,
        }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        if self.ifaddr.is_some() {
            return Ok(self.ifaddr.unwrap());
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "socket is not bound",
        ))
    }

    pub fn close(&self) -> Result<()> {
        let res = shutdown(self.sock, Shutdown::Both);
        udp_socket_closewait();
        res
    }

    pub fn bind(&mut self, ifaddr: SocketAddr) -> Result<()> {
        let sock_addr = stdaddr_to_nixaddr(ifaddr);
        let res = bind(self.sock, &sock_addr);
        if res.is_ok() {
            self.ifaddr = Some(ifaddr);
        }
        res
    }

    pub fn send_to(&self, buf: &[u8], to_addr: SocketAddr) -> Result<usize> {
        let flags = MsgFlags::empty();
        let sock_addr = stdaddr_to_nixaddr(to_addr);
        sendto(self.sock, buf, &sock_addr, flags)
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, Option<socket::SockaddrIn>)> {
        recvfrom::<socket::SockaddrIn>(self.sock, buf)
    }

    pub fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> Result<()> {
        let opt = IpMembershipRequest::new(multiaddr, Some(interface));
        setsockopt(self.sock, IpAddMembership, &opt)
    }
}
