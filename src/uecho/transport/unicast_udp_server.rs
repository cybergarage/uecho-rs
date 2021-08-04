// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use hex::*;
use log::*;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::thread;

use crate::uecho::log::logger;
use crate::uecho::protocol::message::Message;
use crate::uecho::transport::default::{MAX_PACKET_SIZE, PORT};
use crate::uecho::transport::notifier::*;
use crate::uecho::transport::notify_manager::*;
use crate::uecho::transport::observer::*;

pub struct UnicastUdpServer {
    socket: Option<Arc<UdpSocket>>,
    notifier: Notifier,
}

impl UnicastUdpServer {
    pub fn new() -> UnicastUdpServer {
        UnicastUdpServer {
            socket: None,
            notifier: notifier_new(),
        }
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        self.notifier.lock().unwrap().add_observer(observer)
    }

    pub fn send_message(&self, to_addr: SocketAddr, msg: &Message) -> bool {
        let msg_bytes = msg.bytes();
        // match &self.socket {
        //     Some(socket) => {
        //         if socket.send_to(&msg_bytes, "localhost:3610").is_err() {
        //             return false;
        //         }
        //     }
        //     None => return false,
        // }
        let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind host socket");
        if socket.send_to(&msg_bytes, to_addr).is_err() {
            let addr = to_addr.ip();
            let port = to_addr.port();
            warn!("Couldn't send message to {} {}", addr, port);
            return false;
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
        let addr = format!("127.0.0.1:{}", PORT);
        let socket_res = UdpSocket::bind(addr);
        if socket_res.is_err() {
            return false;
        }
        self.socket = Some(Arc::new(socket_res.ok().unwrap()));
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
