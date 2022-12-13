// Copyright (C) 2022 Satoshi Konno All rights reserved.
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

#![allow(dead_code)]

use log::*;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, RwLock};
use std::thread;

use crate::protocol::Message;
use crate::transport::default::*;
use crate::transport::notifier::*;
use crate::transport::observer::ObserverObject;
use crate::transport::udp_socket::UdpSocket;

pub struct UnicastServer {
    socket: Arc<RwLock<UdpSocket>>,
    notifier: Notifier,
}

impl UnicastServer {
    pub fn new() -> UnicastServer {
        UnicastServer {
            socket: Arc::new(RwLock::new(UdpSocket::new())),
            notifier: notifier_new(),
        }
    }

    pub fn add_observer(&mut self, observer: ObserverObject) -> bool {
        self.notifier.lock().unwrap().add_observer(observer)
    }

    pub fn send(&self, to_addr: SocketAddr, msg: &Message) -> bool {
        let msg_bytes = msg.bytes();
        let addr = to_addr.ip();
        let port = to_addr.port();
        info!(
            "SEND {} -> {}:{} ({})",
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
            warn!("Couldn't send message to {} {}", addr, port);
            return false;
        }
        true
    }

    pub fn ifaddr(&self) -> io::Result<SocketAddr> {
        self.socket.read().unwrap().local_addr()
    }

    pub fn bind(&mut self, ifaddr: IpAddr) -> bool {
        let addr = format!("{}:{}", ifaddr, PORT).parse();
        if addr.is_err() {
            error!("bind {} {}", ifaddr, PORT);
            return false;
        }
        let addr: SocketAddr = addr.unwrap();
        debug!("BIND UDP {}", addr);
        if self.socket.write().unwrap().bind(addr).is_err() {
            return false;
        }
        true
    }

    pub fn close(&self) -> bool {
        self.socket.read().unwrap().close();
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
                                remote_addr.ip(),
                                n_bytes,
                                hex::encode_upper(recv_msg)
                            );
                            continue;
                        }
                        info!(
                            "RECV {} -> {} ({})",
                            remote_addr.ip(),
                            socket.read().unwrap().local_addr().ok().unwrap(),
                            msg
                        );
                        msg.set_from(remote_addr.clone());
                        notifier.lock().unwrap().notify(&msg);
                    }
                    Err(e) => {
                        warn!(
                            "RECV {} ({})",
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

    pub fn stop(&self) -> bool {
        if !self.close() {
            return false;
        }
        true
    }
}

impl Drop for UnicastServer {
    fn drop(&mut self) {
        self.stop();
    }
}
