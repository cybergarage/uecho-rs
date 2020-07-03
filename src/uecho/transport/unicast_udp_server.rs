// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::unicast_udp_worker::UnicastUdpWorker;
use std::io;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;

//use std::ptr;

pub struct UnicastUdpServer {
    socket: Option<UdpSocket>,
    worker: Option<UnicastUdpWorker>,
}

impl UnicastUdpServer {
    pub fn new() -> UnicastUdpServer {
        UnicastUdpServer {
            socket: None,
            worker: None,
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
        let addr = format!("localhost:{}", 3690);
        let socket = UdpSocket::bind(addr);
        if socket.is_err() {
            return false;
        }
        self.socket = socket.ok();
        self.worker = Some(UnicastUdpWorker::new());
        true
    }

    pub fn stop(&mut self) -> bool {
        if self.socket.is_some() {
            self.socket = None;
            return true;
        }
        if self.worker.is_some() {
            self.worker = None;
            return true;
        }
        true
    }
}
