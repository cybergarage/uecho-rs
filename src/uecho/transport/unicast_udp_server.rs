// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::Message;
use crate::uecho::protocol::message::*;
use crate::uecho::protocol::message_handler::MessageHandler;
use crate::uecho::transport::unicast_udp_worker::UnicastUdpWorker;
use std::io;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::rc::Weak;
use std::thread;
use std::thread::Builder;
use std::thread::JoinHandle;

pub struct UnicastUdpServer {
    socket: Option<UdpSocket>,
    worker: Option<UnicastUdpWorker>,
    runnable: bool,
    //msg_handler: Option<MessageHandler>,
    //msg_handler: Option<MessageHandler>,
}

impl UnicastUdpServer {
    pub fn new() -> UnicastUdpServer {
        UnicastUdpServer {
            socket: None,
            worker: None,
            runnable: false,
            //msg_handler: None,
            // Weak::new(),
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
        self.runnable = true;
        let thread = thread::spawn(|| {
            let addr = format!("localhost:{}", 3690);
            let socket = UdpSocket::bind(addr);
            if socket.is_err() {
                return;
            }
            let mut buf = [0 as u8; 1500];
            let socket_ref = &socket.ok();
            loop {
                match socket_ref {
                    Some(socket) => match socket.recv_from(&mut buf) {
                        Ok((n_bytes, remote_addr)) => {
                            let mut msg = Message::new();
                            if !msg.parse(&buf[0..n_bytes]) {
                                continue;
                            }
                        }
                        Err(_) => {
                            break;
                        }
                    },
                    None => {
                        break;
                    }
                }
            }
        });
        true
    }

    pub fn stop(&mut self) -> bool {
        self.runnable = false;
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
