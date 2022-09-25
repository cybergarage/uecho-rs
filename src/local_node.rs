// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::Mutex;

// use crate::local_node_observer::LocalNodeObserver;
use crate::object::*;
use crate::protocol::message::Message;
use crate::transport::manager::*;
use crate::transport::observer::*;

pub type TID = u16;
const TID_MIN: TID = 0;
const TID_MAX: TID = 65535;

pub struct LocalNode {
    transport_mgr: Manager,
    objects: Vec<Object>,
    last_tid: TID,
}

impl LocalNode {
    pub fn new() -> Arc<Mutex<LocalNode>> {
        let node = Arc::new(Mutex::new(LocalNode {
            transport_mgr: Manager::new(),
            objects: Vec::new(),
            last_tid: TID_MIN,
        }));
        node.lock()
            .unwrap()
            .add_observer(Arc::new(Mutex::new(node.clone())));
        node
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

    pub fn objects(&self) -> &Vec<Object> {
        return &self.objects;
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

    pub fn post_message(&self, to_addr: SocketAddr, msg: &Message) -> bool {
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

    fn next_tid(&mut self) -> TID {
        if TID_MAX <= self.last_tid {
            self.last_tid = TID_MIN;
        } else {
            self.last_tid += 1;
        }
        self.last_tid
    }
}

impl Observer for Arc<Mutex<LocalNode>> {
    fn message_received(&mut self, _msg: &Message) {
        // let mut node = self.lock().unwrap();
        // let obj = Object::new();
        // node.add_object(obj);
    }
}
