// Copyright (C) 2022 The uecho-rs Authors All rights reserved.
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

use log::info;
use std::net::SocketAddr;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;

use crate::database::StandardDatabase;
use crate::local_node::*;
use crate::message::SearchMessage;
use crate::node_profile::*;
use crate::object::*;
use crate::protocol::message::*;
use crate::remote_node::*;
use crate::transport::default::PORT;
use crate::transport::observer::*;

pub struct ControllerObserver {
    db: StandardDatabase,
    node: Arc<Mutex<LocalNode>>,
    pub remote_nodes: Vec<RemoteNode>,
}

impl ControllerObserver {
    pub fn new() -> Arc<Mutex<ControllerObserver>> {
        ControllerObserver::new_with_node(LocalNode::new())
    }

    pub fn new_with_node(node: Arc<Mutex<LocalNode>>) -> Arc<Mutex<ControllerObserver>> {
        let ctrl = Arc::new(Mutex::new(ControllerObserver {
            db: StandardDatabase::new(),
            node: node,
            remote_nodes: Vec::new(),
        }));
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

    pub fn local_node(&self) -> Arc<Mutex<LocalNode>> {
        self.node.clone()
    }

    pub fn nodes(&self) -> &Vec<RemoteNode> {
        return &self.remote_nodes;
    }

    pub fn search_object(&mut self, obj_code: ObjectCode) -> bool {
        let mut msg = SearchMessage::new();
        msg.set_deoj(obj_code);
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

        if !msg.is_node_profile_message() {
            return;
        }

        let mut remote_node = RemoteNode::from_message(msg);
        info!("FOUND: {}", remote_node.addr());
        // Copy standard object properties
        for (n, obj) in remote_node.objects_mut().iter_mut().enumerate() {
            let std_obj = ctrl.db.find_object(obj.code());
            if std_obj.is_none() {
                continue;
            }
            info!("    [{}] {:02X}", n, std_obj.unwrap().code());
            for std_prop in std_obj.unwrap().properties() {
                obj.add_property(std_prop.clone());
            }
        }
        ctrl.add_remote_node(remote_node);
    }
}
