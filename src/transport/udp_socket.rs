// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::transport::error::{BindError, ScoketError};
use crate::transport::result::Result;
use log::warn;
use nix::sys::socket;
use nix::sys::socket::sockopt::{
    IpAddMembership, IpMulticastLoop, Ipv6AddMembership, ReuseAddr, ReusePort,
};
use nix::sys::socket::{bind, recvfrom, sendto, setsockopt, shutdown, socket};
use nix::sys::socket::{
    AddressFamily, IpMembershipRequest, Ipv6MembershipRequest, MsgFlags, Shutdown, SockFlag,
    SockType, SockaddrIn,
};
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::os::unix::io::{AsRawFd, RawFd};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

pub struct UdpSocket {
    sock: Option<std::net::UdpSocket>,
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
        UdpSocket {
            sock: None,
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

    pub fn close(&mut self) {
        if self.sock.is_none() {
            return;
        }
        let fd = self.sock.as_ref().unwrap().as_raw_fd();
        let res = shutdown(fd, Shutdown::Both);
        if res.is_err() {
            warn!("close {:?}", res.err());
        }
        self.sock = None;
        thread::sleep(Duration::from_secs(1));
    }

    pub fn bind(&mut self, ifaddr: SocketAddr) -> Result<()> {
        if self.sock.is_some() {
            self.close();
        }
        let sock = std::net::UdpSocket::bind(ifaddr.to_string());
        if sock.is_err() {
            return Err(ScoketError::new(&format!("could not bind to {}", ifaddr)));
        }
        let fd = sock.as_ref().unwrap().as_raw_fd();
        if setsockopt(fd, ReuseAddr, &true).is_err() {
            warn!("SO_REUSEADDR is not supported");
        }
        if setsockopt(fd, ReusePort, &true).is_err() {
            warn!("SO_REUSEPORT is not supported");
        }
        self.sock = Some(sock.unwrap());
        Ok(())
    }

    pub fn send_to(&self, buf: &[u8], to_addr: SocketAddr) -> Result<usize> {
        if self.sock.is_none() {
            return Err(BindError::new());
        }
        let res = self.sock.as_ref().unwrap().send_to(buf, to_addr);
        if res.is_err() {
            return Err(ScoketError::new(&format!("{:?}", res.err())));
        }
        Ok(res.unwrap())
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        if self.sock.is_none() {
            return Err(BindError::new());
        }
        self.sock.as_ref().unwrap().recv_from(buf)
    }

    pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> Result<()> {
        if self.sock.is_none() {
            return Err(BindError::new());
        }
        self.sock
            .as_ref()
            .unwrap()
            .join_multicast_v4(multiaddr, interface)
    }

    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, _interface: &Ipv6Addr) -> Result<()> {
        if self.sock.is_none() {
            return Err(BindError::new());
        }
        self.sock.as_ref().unwrap().join_multicast_v6(multiaddr, 0)
    }
}
