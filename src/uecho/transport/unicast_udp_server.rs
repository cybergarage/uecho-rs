// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::Message;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
//use std::ptr;

pub struct UnicastUdpServer {
    socket: Option<UdpSocket>,
}

impl UnicastUdpServer {
    pub fn new() -> UnicastUdpServer {
        UnicastUdpServer { socket: None }
    }

    pub fn send_message<A: ToSocketAddrs>(&self, msg: &Message, addr: A) -> bool {
        // https://doc.rust-lang.org/beta/std/net/struct.UdpSocket.html
        let msg_bytes = msg.bytes();
        true
    }

    pub fn start(&mut self) -> bool {
        let addr = format!("127.0.0.1:{}", 3690);
        let socket = UdpSocket::bind(addr);
        if socket.is_err() {
            return false;
        }

        self.socket = Some(socket.unwrap());

        true
    }

    pub fn stop(&mut self) -> bool {
        if self.socket.is_none() {
            return true;
        }
        self.socket = None;
        true
    }
}
