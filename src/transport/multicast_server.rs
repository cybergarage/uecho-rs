// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use log::*;
use std::io;
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::thread;

use crate::protocol::message::Message;
use crate::transport::default::*;
use crate::transport::notifier::*;
use crate::transport::notify_manager::*;
use crate::transport::observer::*;
//use crate::transport::udp_socket::*;
use crate::transport::udp_socket::UdpSocket;

const ANY_ADDR: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

pub struct MulticastServer {
    socket: Option<Arc<UdpSocket>>,
    notifier: Notifier,
}

impl MulticastServer {
    pub fn new() -> MulticastServer {
        MulticastServer {
            socket: None,
            notifier: notifier_new(),
        }
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        self.notifier.lock().unwrap().add_observer(observer)
    }

    pub fn notify(&self, msg: &Message) -> bool {
        let to_addr_str = format!("{}:{}", MULTICAST_V4_ADDRESS, PORT);
        let to_addr: SocketAddr = to_addr_str.parse().unwrap();
        let msg_bytes = msg.bytes();
        let addr = to_addr.ip();
        let port = to_addr.port();
        match &self.socket {
            Some(socket) => {
                info!(
                    "MCST {} -> {}:{} ({})",
                    socket.local_addr().unwrap(),
                    addr,
                    port,
                    msg,
                );
                if socket.send_to(&msg_bytes, to_addr).is_err() {
                    warn!("Couldn't notify message to {} {}", addr, port);
                    return false;
                }
            }
            None => {
                let any = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
                let mut socket = UdpSocket::new();
                socket.bind(any).expect("failed to bind host socket");
                info!(
                    "MCST {} -> {}:{} ({})",
                    socket.local_addr().unwrap(),
                    addr,
                    port,
                    msg,
                );
                if socket.send_to(&msg_bytes, to_addr).is_err() {
                    warn!("Couldn't notify message to {} {}", addr, port);
                    return false;
                }
            }
        }
        true
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        match &self.socket {
            Some(socket) => {
                return socket.local_addr();
            }
            None => return Err(io::Error::new(io::ErrorKind::NotConnected, "")),
        }
    }

    pub fn bind(&mut self, ifaddr: IpAddr) -> bool {
        let mut socket = UdpSocket::new();
        let addr = format!("{}:{}", ifaddr, PORT);
        let addr: SocketAddr = addr.parse().unwrap();
        debug!("BIND MCT {}", addr);
        if socket.bind(addr).is_err() {
            self.socket = None;
            return false;
        }
        match ifaddr {
            IpAddr::V4(ip4) => {
                if socket.join_multicast_v4(MULTICAST_V4_ADDRESS, ip4).is_err() {
                    self.close();
                    return false;
                }
                debug!("BIND MCT {} ({}:{})", ifaddr, MULTICAST_V4_ADDRESS, ip4);
            }
            IpAddr::V6(_) => return false,
        }
        self.socket = Some(Arc::new(UdpSocket::from(socket)));
        true
    }

    pub fn close(&mut self) -> bool {
        if self.socket.is_some() {
            let socket = self.socket.clone();
            socket.unwrap().close();
            self.socket = None;
        }
        true
    }

    pub fn start(&mut self) -> bool {
        if self.socket.is_none() {
            return false;
        }
        let socket = self.socket.clone().unwrap();
        let notifier = self.notifier.clone();
        thread::spawn(move || {
            let mut buf = [0 as u8; MAX_PACKET_SIZE];
            loop {
                let recv_res = socket.recv_from(&mut buf);
                match &recv_res {
                    Ok((n_bytes, remote_addr)) => {
                        let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
                        let recv_msg = &buf[0..*n_bytes];
                        let mut msg = Message::new();
                        if !msg.parse(recv_msg) {
                            warn!(
                                "Couldn't parse message {} [{}] {}",
                                remote_addr,
                                n_bytes,
                                hex::encode_upper(recv_msg)
                            );
                            continue;
                        }
                        info!(
                            "RECV {} -> {} ({})",
                            remote_addr,
                            socket.local_addr().ok().unwrap(),
                            msg
                        );
                        msg.set_addr(remote_addr.ip());
                        notifier.lock().unwrap().notify(&msg);
                    }
                    Err(e) => {
                        error!("RECV {} -> {}", socket.local_addr().ok().unwrap(), e);
                        break;
                    }
                }
            }
        });
        true
    }

    pub fn stop(&mut self) -> bool {
        if !self.close() {
            return false;
        }
        true
    }
}
