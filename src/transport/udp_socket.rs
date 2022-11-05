// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use log::warn;
use nix::sys::socket;
use nix::sys::socket::sockopt::{IpAddMembership, Ipv6AddMembership, ReuseAddr, ReusePort};
use nix::sys::socket::{bind, recvfrom, sendto, setsockopt, shutdown, socket};
use nix::sys::socket::{
    AddressFamily, IpMembershipRequest, Ipv6MembershipRequest, MsgFlags, Shutdown, SockFlag,
    SockType, SockaddrIn,
};
use nix::Result;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::os::unix::io::RawFd;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

pub struct UdpSocket {
    sock: RawFd,
    ifaddr: Option<SocketAddr>,
}

fn stdaddr_to_nixaddrin(saddr: SocketAddr) -> SockaddrIn {
    let addr = saddr.ip();
    let port = saddr.port();
    SockaddrIn::from_str(&format!("{}:{}", addr, port)).unwrap()
}

fn nixdaddrin_to_ipaddr(addrin: SockaddrIn) -> SocketAddr {
    FromStr::from_str(&addrin.to_string()).unwrap()
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
        thread::sleep(Duration::from_secs(1));
        res
    }

    pub fn bind(&mut self, ifaddr: SocketAddr) -> Result<()> {
        let sock_addr = stdaddr_to_nixaddrin(ifaddr);
        let res = bind(self.sock, &sock_addr);
        if res.is_ok() {
            self.ifaddr = Some(ifaddr);
        }
        res
    }

    pub fn send_to(&self, buf: &[u8], to_addr: SocketAddr) -> Result<usize> {
        let flags = MsgFlags::empty();
        let sock_addr = stdaddr_to_nixaddrin(to_addr);
        sendto(self.sock, buf, &sock_addr, flags)
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, Option<SocketAddr>)> {
        match recvfrom::<socket::SockaddrIn>(self.sock, buf) {
            Ok((n_bytes, remote_addr)) => {
                if remote_addr.is_some() {
                    return Ok((n_bytes, Some(nixdaddrin_to_ipaddr(remote_addr.unwrap()))));
                }
                return Ok((n_bytes, None));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> Result<()> {
        let opt = IpMembershipRequest::new(multiaddr, Some(interface));
        setsockopt(self.sock, IpAddMembership, &opt)
    }

    pub fn join_multicast_v6(&self, multiaddr: Ipv6Addr, interface: Ipv6Addr) -> Result<()> {
        let opt = Ipv6MembershipRequest::new(multiaddr);
        setsockopt(self.sock, Ipv6AddMembership, &opt)
    }
}
