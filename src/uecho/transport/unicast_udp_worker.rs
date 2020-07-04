// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::Message;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::thread;
use std::thread::Builder;
use std::thread::JoinHandle;

pub struct UnicastUdpWorker {
    socket: Option<UdpSocket>,
}

impl UnicastUdpWorker {
    pub fn new() -> UnicastUdpWorker {
        UnicastUdpWorker { socket: None }
    }

    pub fn start(&mut self, socket: UdpSocket) -> bool {
        let thread = thread::spawn(move || {
            let mut buf = [0 as u8; 1500];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((n_bytes, remote_addr)) => {}
                    Err(_) => break,
                }
            }
        });
        true
    }

    pub fn stop(&mut self) -> bool {
        true
    }
}

impl Drop for UnicastUdpWorker {
    fn drop(&mut self) {}
}
