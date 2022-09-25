// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::SocketAddr;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;

use crate::local_node::*;
use crate::message::*;
use crate::node_profile::*;
use crate::object::*;
use crate::protocol::esv::*;
use crate::protocol::message::*;
use crate::remote_node::*;
use crate::transport::default::PORT;
use crate::transport::observer::*;

pub struct ControllerObserver {
    node: Arc<Mutex<LocalNode>>,
    pub remote_nodes: Vec<RemoteNode>,
}

impl ControllerObserver {
    pub fn new() -> Arc<Mutex<ControllerObserver>> {
        let ctrl = Arc::new(Mutex::new(ControllerObserver {
            node: LocalNode::new(),
            remote_nodes: Vec::new(),
        }));
        ctrl.lock()
            .unwrap()
            .add_observer(Arc::new(Mutex::new(ctrl.clone())));
        ctrl
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        let mut node = self.node.lock().unwrap();
        node.add_observer(observer.clone())
    }

    pub fn add_remote_node(&mut self, node: RemoteNode) -> bool {
        for found_node in self.remote_nodes.iter() {
            if found_node == &node {
                return false;
            }
        }
        self.remote_nodes.push(node);
        true
    }

    pub fn nodes(&self) -> &Vec<RemoteNode> {
        return &self.remote_nodes;
    }

    pub fn search_object(&mut self, obj_code: ObjectCode) -> bool {
        let mut msg = message_serarch_new();
        msg.set_destination_object_code(obj_code);
        let mut node = self.node.lock().unwrap();
        node.notify(&mut msg)
    }

    pub fn search_all(&mut self) -> bool {
        self.search_object(NODE_PROFILE_OBJECT_CODE)
    }

    pub fn send_message(&self, remote_node: &RemoteNode, msg: &mut Message) -> bool {
        let mut node = self.node.lock().unwrap();
        node.send_message(SocketAddr::new(remote_node.addr(), PORT), msg)
    }

    pub fn post_message(&self, remote_node: &RemoteNode, msg: &mut Message) -> Receiver<Message> {
        let mut node = self.node.lock().unwrap();
        node.post_message(SocketAddr::new(remote_node.addr(), PORT), msg)
    }

    pub fn start(&mut self) -> bool {
        let mut node = self.node.lock().unwrap();
        if !node.start() {
            return false;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        let mut node = self.node.lock().unwrap();
        if !node.stop() {
            return false;
        }
        true
    }
}

impl Observer for Arc<Mutex<ControllerObserver>> {
    fn message_received(&mut self, msg: &Message) {
        let mut ctrl = self.lock().unwrap();
        let remote_node = RemoteNode::from_message(msg);

        fn is_node_profile_message(msg: &Message) -> bool {
            let esv = msg.esv();
            if esv != Esv::Notification && esv != Esv::ReadResponse {
                return false;
            }
            let dst_obj = msg.destination_object_code();
            if dst_obj != NODE_PROFILE_OBJECT_CODE && dst_obj != NODE_PROFILE_OBJECT_READ_ONLY {
                return false;
            }
            true
        }

        if is_node_profile_message(msg) {
            ctrl.add_remote_node(remote_node);
        }
    }
}
