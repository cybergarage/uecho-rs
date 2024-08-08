// Copyright (C) 2022 The uecho-rs Authors All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// NOTE: Standard UdpSocket could not enable SO_REUSEADDR
// use nix::sys::socket::sockopt::{IpMulticastLoop, ReuseAddr, ReusePort};
use crate::transport::error::{BindError, ScoketError};
use crate::transport::result::Result;
use crate::transport::PORT;
#[cfg(feature = "unix")]
use log::warn;
#[cfg(feature = "unix")]
use nix::sys::socket::{shutdown, Shutdown};
#[cfg(feature = "unix")]
use nix::unistd::close;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
#[cfg(feature = "unix")]
use std::os::unix::io::AsRawFd;
use std::{thread, time};

const UDP_SOCKET_BIND_RETRY_MAX: u32 = 3;
const UDP_SOCKET_BIND_SLEEP_MSEC: u64 = 100;

pub struct UdpSocket {
    sock: Option<std::net::UdpSocket>,
    ifaddr: Option<SocketAddr>,
}

#[cfg(feature = "unix")]
fn create_socket_v4(ifaddr: SocketAddr) -> io::Result<std::net::UdpSocket> {
    net2::UdpBuilder::new_v4()?
        .reuse_address(true)?
        // .reuse_port(true)?
        .bind(ifaddr)
}

#[cfg(not(feature = "unix"))]
fn create_socket_v4(ifaddr: SocketAddr) -> io::Result<std::net::UdpSocket> {
    let sock = std::net::UdpSocket::bind(ifaddr.to_string());
    sock
}

#[cfg(feature = "unix")]
fn create_socket_v6(ifaddr: SocketAddr) -> io::Result<std::net::UdpSocket> {
    net2::UdpBuilder::new_v6()?
        .reuse_address(true)?
        // .reuse_port(true)?
        .bind(ifaddr)
}

#[cfg(not(feature = "unix"))]
fn create_socket_v6(ifaddr: SocketAddr) -> io::Result<std::net::UdpSocket> {
    let sock = std::net::UdpSocket::bind(ifaddr.to_string());
    sock
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

        let mut sock: Option<std::net::UdpSocket> = None;

        for _n in 0..UDP_SOCKET_BIND_RETRY_MAX {
            if ifaddr.is_ipv4() {
                let sock_v4 = create_socket_v4(ifaddr);
                if sock_v4.is_ok() {
                    sock = Some(sock_v4.unwrap());
                    break;
                }
            } else if ifaddr.is_ipv6() {
                let sock_v6 = create_socket_v6(ifaddr);
                if sock_v6.is_ok() {
                    sock = Some(sock_v6.unwrap());
                    break;
                }
            }
            thread::sleep(time::Duration::from_millis(UDP_SOCKET_BIND_SLEEP_MSEC));
        }

        if sock.is_none() {
            return Err(ScoketError::new(&format!("could not bind to {}", ifaddr)));
        }

        self.sock = sock;
        self.ifaddr = Some(ifaddr);
        Ok(())
    }

    pub fn close(&self) -> bool {
        if self.sock.is_none() {
            return true;
        }
        #[cfg(feature = "unix")]
        {
            let fd = self.sock.as_ref().unwrap().as_raw_fd();
            let res = shutdown(fd, Shutdown::Both);
            if res.is_err() {
                warn!("shutdown ({})", res.err().unwrap().desc());
            }
            let res = close(fd);
            if res.is_err() {
                warn!("close {:?}", res.err());
                return false;
            }
        }
        thread::sleep(time::Duration::from_millis(UDP_SOCKET_BIND_SLEEP_MSEC));
        true
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

impl Drop for UdpSocket {
    fn drop(&mut self) {
        self.close();
    }
}
