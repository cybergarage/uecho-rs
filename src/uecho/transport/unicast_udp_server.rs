// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::io;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::Arc;
use std::thread;

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::default::{MAX_PACKET_SIZE, PORT};
use crate::uecho::transport::notify_manager::*;
use crate::uecho::transport::observer::*;

pub struct UnicastUdpServer {
    socket: Option<Arc<UdpSocket>>,
    observers: Observers,
}

impl UnicastUdpServer {
    pub fn new() -> UnicastUdpServer {
        UnicastUdpServer {
            socket: None,
            observers: observer_new(),
        }
    }
    pub fn send_message<A: ToSocketAddrs>(&self, addr: A, msg: &Message) -> bool {
        match &self.socket {
            Some(socket) => {
                let msg_bytes = msg.bytes();
                if socket.send_to(&msg_bytes, addr).is_err() {
                    return false;
                }
            }
            None => return false,
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

    pub fn start(&mut self) -> bool {
        let addr = format!("localhost:{}", PORT);
        let socket_res = UdpSocket::bind(addr);
        if socket_res.is_err() {
            return false;
        }
        self.socket = Some(Arc::new(socket_res.ok().unwrap()));
        let socket = self.socket.clone();
        thread::spawn(move || {
            let mut buf = [0 as u8; MAX_PACKET_SIZE];
            let socket = socket.unwrap();
            loop {
                let recv_res = socket.recv_from(&mut buf);
                match &recv_res {
                    Ok((n_bytes, _remote_addr)) => {
                        let mut msg = Message::new();
                        if !msg.parse(&buf[0..*n_bytes]) {
                            continue;
                        }
                    }
                    Err(_) => {
                        break;
                    } // FIXME:
                      // self.notify(&msg);
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

impl NotifytManager for UnicastUdpServer {
    fn observers(&mut self) -> &Observers {
        return &self.observers;
    }
}
