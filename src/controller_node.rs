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

use crate::message::SearchMessage;
use crate::node::Node;
use crate::node_profile::{NodeProfile, NODE_PROFILE_OBJECT_CODE};
use crate::object::ObjectCode;
use crate::protocol::Message;
use crate::remote_node::RemoteNode;
use crate::super_object::SUPER_OBJECT_CODE;
use crate::transport::PORT;
use crate::transport::{Observer, ObserverObject};

pub struct ControllerNode {
    node: Arc<Mutex<Node>>,
    pub remote_nodes: Vec<RemoteNode>,
}

impl ControllerNode {
    pub fn new() -> Arc<Mutex<ControllerNode>> {
        ControllerNode::new_with_node(Node::new())
    }

    pub fn new_with_node(node: Arc<Mutex<Node>>) -> Arc<Mutex<ControllerNode>> {
        let ctrl = Arc::new(Mutex::new(ControllerNode {
            node: node,
            remote_nodes: Vec::new(),
        }));
        ctrl
    }

    pub fn add_observer(&mut self, observer: ObserverObject) -> bool {
        let mut node = self.node.lock().unwrap();
        node.add_observer(observer.clone())
    }

    pub fn add_remote_node(&mut self, node: RemoteNode) -> bool {
        self.remote_nodes.push(node);
        true
    }

    pub fn replace_remote_node(&mut self, node: RemoteNode) -> bool {
        for (n, found_node) in self.remote_nodes.iter().enumerate() {
            if found_node != &node {
                continue;
            }
            self.remote_nodes.remove(n);
            break;
        }
        self.remote_nodes.push(node);
        true
    }

    pub fn node(&self) -> Arc<Mutex<Node>> {
        self.node.clone()
    }

    pub fn nodes(&self) -> &Vec<RemoteNode> {
        return &self.remote_nodes;
    }

    pub fn search_object(&mut self, obj_code: ObjectCode) -> bool {
        let mut msg = SearchMessage::new();
        msg.set_seoj(NODE_PROFILE_OBJECT_CODE);
        msg.set_deoj(obj_code);
        let mut node = self.node.lock().unwrap();
        node.notify(&mut msg)
    }

    pub fn search_all(&mut self) -> bool {
        self.search_object(NODE_PROFILE_OBJECT_CODE)
    }

    pub fn send_message(&self, remote_node: &RemoteNode, msg: &mut Message) -> bool {
        msg.set_seoj(NODE_PROFILE_OBJECT_CODE);
        let mut node = self.node.lock().unwrap();
        node.send_message(SocketAddr::new(remote_node.addr(), PORT), msg)
    }

    pub fn post_message(&self, remote_node: &RemoteNode, msg: &mut Message) -> Receiver<Message> {
        msg.set_seoj(NODE_PROFILE_OBJECT_CODE);
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

impl Observer for Arc<Mutex<ControllerNode>> {
    fn message_received(&mut self, msg: &Message) {
        let mut ctrl = self.lock().unwrap();

        if !msg.is_node_profile_message() {
            return;
        }

        let mut remote_node = RemoteNode::from_message(msg);
        info!("FOUND: {}", remote_node.addr());
        for (_, obj) in remote_node.objects_mut().iter_mut().enumerate() {
            obj.add_standard_properties(SUPER_OBJECT_CODE);
            obj.add_standard_properties(obj.code());
        }
        remote_node.add_object(NodeProfile::new());
        ctrl.replace_remote_node(remote_node);
    }
}
