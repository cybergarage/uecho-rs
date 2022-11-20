// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// NOTE: Standard UdpSocket could not enable SO_REUSEADDR
// use nix::sys::socket::sockopt::{IpMulticastLoop, ReuseAddr, ReusePort};
use crate::transport::default::PORT;
use crate::transport::error::{BindError, ScoketError};
use crate::transport::result::Result;
use log::warn;
use nix::sys::socket::{setsockopt, shutdown, Shutdown};
use nix::unistd::close;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::os::unix::io::AsRawFd;
use std::thread;
use std::time::Duration;

pub struct UdpSocket {
    sock: Option<std::net::UdpSocket>,
    ifaddr: Option<SocketAddr>,
}

fn create_socket(ifaddr: SocketAddr) -> io::Result<std::net::UdpSocket> {
    net2::UdpBuilder::new_v4()?
        .reuse_address(true)?
        // .reuse_port(true)?
        .bind(ifaddr)
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
    pub fn bind(&mut self, ifaddr: SocketAddr) -> Result<()> {
        if self.sock.is_some() {
            self.close();
        }

        // NOTE: Standard UdpSocket could not enable SO_REUSEADDR
        // let sock = std::net::UdpSocket::bind(ifaddr.to_string());
        // let fd = sock.as_ref().unwrap().as_raw_fd();
        // if setsockopt(fd, ReuseAddr, &true).is_err() {
        //     warn!("SO_REUSEADDR is not supported");
        // }
        // if setsockopt(fd, ReusePort, &true).is_err() {
        //     warn!("SO_REUSEPORT is not supported");
        // }
        // if setsockopt(fd, IpMulticastLoop, &true).is_err() {
        //     warn!("IP_MULTICAST_LOOP is not supported");
        // }

        // net2::UdpBuilder could enable SO_REUSEADDR and SO_REUSEPORT on macOS and Linux
        let sock = create_socket(ifaddr);
        if sock.is_err() {
            return Err(ScoketError::new(&format!("could not bind to {}", ifaddr)));
        }

        self.sock = Some(sock.unwrap());
        self.ifaddr = Some(ifaddr);
        Ok(())
    }

    pub fn close(&self) {
        if self.sock.is_none() {
            return;
        }
        let fd = self.sock.as_ref().unwrap().as_raw_fd();
        let res = shutdown(fd, Shutdown::Both);
        if res.is_err() {
            warn!("shutdown {:?}", res.err());
        }
        let res = close(fd);
        if res.is_err() {
            warn!("close {:?}", res.err());
        }
        thread::sleep(Duration::from_secs(1));
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

    pub fn join_multicast_v4(&mut self, multiaddr: &Ipv4Addr, ifaddr: &Ipv4Addr) -> Result<()> {
        if self.sock.is_none() {
            return Err(BindError::new());
        }
        self.ifaddr = Some(format!("{}:{}", ifaddr, PORT).parse().unwrap());
        self.sock
            .as_ref()
            .unwrap()
            .join_multicast_v4(multiaddr, ifaddr)
    }

    pub fn join_multicast_v6(&mut self, multiaddr: &Ipv6Addr, ifaddr: &Ipv6Addr) -> Result<()> {
        if self.sock.is_none() {
            return Err(BindError::new());
        }
        self.ifaddr = Some(format!("{}:{}", ifaddr, PORT).parse().unwrap());
        self.sock.as_ref().unwrap().join_multicast_v6(multiaddr, 0)
    }
}
