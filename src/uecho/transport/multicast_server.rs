// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use log::*;
use std::io;
use std::net::IpAddr;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::thread;

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::default::*;
use crate::uecho::transport::notifier::*;
use crate::uecho::transport::notify_manager::*;
use crate::uecho::transport::observer::*;

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
                if socket.send_to(&msg_bytes, to_addr).is_err() {
                    warn!("Couldn't send message to {} {}", addr, port);
                    return false;
                }
            }
            None => {
                let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind host socket");
                if socket.send_to(&msg_bytes, to_addr).is_err() {
                    warn!("Couldn't send message to {} {}", addr, port);
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
        self.socket = None;
        let addr = format!("{}:{}", ifaddr, PORT);
        let socket_res = UdpSocket::bind(addr);
        if socket_res.is_err() {
            return false;
        }
        let socket = socket_res.ok().unwrap();
        match ifaddr {
            IpAddr::V4(ip4) => {
                if socket
                    .join_multicast_v4(&MULTICAST_V4_ADDRESS, &ip4)
                    .is_err()
                {
                    self.close();
                    return false;
                }
            }
            IpAddr::V6(_) => return false,
        }
        self.socket = Some(Arc::new(socket));
        true
    }

    pub fn close(&mut self) -> bool {
        if self.socket.is_some() {
            self.socket = None;
        }
        true
    }

    pub fn start(&mut self) -> bool {
        if self.socket.is_none() {
            return false;
        }
        let socket = self.socket.clone();
        let notifier = self.notifier.clone();
        thread::spawn(move || {
            let mut buf = [0 as u8; MAX_PACKET_SIZE];
            let socket = socket.unwrap();
            loop {
                let recv_res = socket.recv_from(&mut buf);
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
                        notifier.lock().unwrap().notify(&msg);
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        });
        true
    }

    pub fn stop(&mut self) -> bool {
        if self.socket.is_some() {
            self.socket = None;
            return true;
        }
        true
    }
}
