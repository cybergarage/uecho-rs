// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::object::*;
use crate::protocol::message::Message;
use crate::transport::manager::*;
use crate::transport::observer::*;

pub struct LocalNode {
    transport_mgr: Manager,
    objects: Vec<Object>,
}

impl LocalNode {
    pub fn new() -> LocalNode {
        LocalNode {
            transport_mgr: Manager::new(),
            objects: Vec::new(),
        }
    }

    pub fn addr(&self) -> IpAddr {
        match self.transport_mgr.local_addr() {
            Ok(local_addr) => return local_addr.ip(),
            Err(_) => return IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }

    pub fn add_object(&mut self, obj: Object) -> bool {
        self.objects.push(obj);
        true
    }

    pub fn get_object(&self, code: ObjectCode) -> Option<&Object> {
        for n in 0..self.objects.len() {
            if self.objects[n].code() == code {
                return Some(&self.objects[n]);
            }
        }
        None
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        self.transport_mgr.add_observer(observer.clone())
    }

    pub fn send_message(&self, to_addr: SocketAddr, msg: &Message) -> bool {
        self.transport_mgr.send(to_addr, msg)
    }

    pub fn notify(&self, msg: &Message) -> bool {
        self.transport_mgr.notify(msg)
    }

    pub fn start(&mut self) -> bool {
        if !self.transport_mgr.start() {
            return false;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        if !self.transport_mgr.stop() {
            return false;
        }
        true
    }
}