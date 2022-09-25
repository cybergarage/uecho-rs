// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;

// use crate::local_node_observer::LocalNodeObserver;
use crate::node_profile::*;
use crate::object::*;
use crate::protocol::message::*;
use crate::transport::manager::*;
use crate::transport::observer::*;

pub struct LocalNode {
    transport_mgr: Manager,
    objects: Vec<Object>,
    last_tid: TID,
    post_sender: Sender<Message>,
}

impl LocalNode {
    pub fn new() -> Arc<Mutex<LocalNode>> {
        let (tx, _): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let node = Arc::new(Mutex::new(LocalNode {
            transport_mgr: Manager::new(),
            objects: Vec::new(),
            last_tid: TID_MIN,
            post_sender: tx,
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

    pub fn send_message(&mut self, to_addr: SocketAddr, msg: &mut Message) -> bool {
        self.update_message_header(msg);
        self.transport_mgr.send(to_addr, msg)
    }

    pub fn post_message(&mut self, to_addr: SocketAddr, msg: &mut Message) -> Receiver<Message> {
        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        self.post_sender = tx;
        self.transport_mgr.send(to_addr, msg);
        rx
    }

    pub fn notify(&mut self, msg: &mut Message) -> bool {
        self.update_message_header(msg);
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

    fn update_message_header(&mut self, msg: &mut Message) {
        msg.set_tid(self.next_tid());
        msg.set_source_object_code(NODE_PROFILE_OBJECT_CODE);
    }

    fn next_tid(&mut self) -> TID {
        if TID_MAX <= self.last_tid {
            self.last_tid = TID_MIN;
        } else {
            self.last_tid += 1;
        }
        self.last_tid
    }

    fn is_last_message_response(&self, msg: &Message) -> bool {
        if msg.tid() != self.last_tid {
            return false;
        }
        true
    }

    fn send_post_reopnse(&self, msg: Message) {
        self.post_sender.send(msg).unwrap();
    }
}

impl Observer for Arc<Mutex<LocalNode>> {
    fn message_received(&mut self, msg: &Message) {
        let node = self.lock().unwrap();
        if node.is_last_message_response(msg) {
            node.send_post_reopnse(msg.clone());
        }
    }
}
