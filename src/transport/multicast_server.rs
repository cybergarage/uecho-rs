// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use log::*;
use std::io;
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, RwLock};
use std::thread;

use crate::protocol::message::Message;
use crate::transport::default::*;
use crate::transport::notifier::*;
use crate::transport::notify_manager::*;
use crate::transport::observer::*;
use crate::transport::udp_socket::UdpSocket;

pub struct MulticastServer {
    socket: Arc<RwLock<UdpSocket>>,
    notifier: Notifier,
}

impl MulticastServer {
    pub fn new() -> MulticastServer {
        MulticastServer {
            socket: Arc::new(RwLock::new(UdpSocket::new())),
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
        info!(
            "MCST {} -> {}:{} ({})",
            self.socket.read().unwrap().local_addr().unwrap(),
            addr,
            port,
            msg,
        );
        if self
            .socket
            .read()
            .unwrap()
            .send_to(&msg_bytes, to_addr)
            .is_err()
        {
            warn!("Couldn't notify message to {} {}", addr, port);
            return false;
        }
        true
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.read().unwrap().local_addr()
    }

    pub fn bind(&mut self, ifaddr: IpAddr) -> bool {
        let addr = format!("{}:{}", ifaddr, PORT);
        let addr: SocketAddr = addr.parse().unwrap();
        debug!("BIND MCT {}", addr);
        if self.socket.write().unwrap().bind(addr).is_err() {
            return false;
        }
        match ifaddr {
            IpAddr::V4(ip4) => {
                if self
                    .socket
                    .read()
                    .unwrap()
                    .join_multicast_v4(&MULTICAST_V4_ADDRESS, &ip4)
                    .is_err()
                {
                    self.close();
                    return false;
                }
                debug!(
                    "BIND MCT {}:{} -> {}:{}",
                    ifaddr, PORT, MULTICAST_V4_ADDRESS, ip4
                );
            }
            IpAddr::V6(ip6) => {
                if self
                    .socket
                    .read()
                    .unwrap()
                    .join_multicast_v6(&MULTICAST_V6_ADDRESS, &ip6)
                    .is_err()
                {
                    self.close();
                    return false;
                }
                debug!(
                    "BIND MCT {}:{} -> {}:{}",
                    ifaddr, PORT, MULTICAST_V6_ADDRESS, ip6
                );
            }
        }
        true
    }

    pub fn close(&mut self) -> bool {
        self.socket.write().unwrap().close();
        true
    }

    pub fn start(&mut self) -> bool {
        let socket = self.socket.clone();
        let notifier = self.notifier.clone();
        thread::spawn(move || {
            let mut buf = [0 as u8; MAX_PACKET_SIZE];
            loop {
                let recv_res = socket.read().unwrap().recv_from(&mut buf);
                match &recv_res {
                    Ok((n_bytes, remote_addr)) => {
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
                            socket.read().unwrap().local_addr().ok().unwrap(),
                            msg
                        );
                        msg.set_addr(remote_addr.ip());
                        notifier.lock().unwrap().notify(&msg);
                    }
                    Err(e) => {
                        error!(
                            "RECV {} -> {}",
                            socket.read().unwrap().local_addr().ok().unwrap(),
                            e
                        );
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
