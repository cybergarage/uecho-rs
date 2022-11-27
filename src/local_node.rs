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

use crate::node_profile::*;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;

use crate::object::*;
use crate::protocol::{Message, TID, TID_MAX, TID_MIN};
use crate::transport::Manager;
use crate::transport::*;

/// LocalNode represents an internal ECHONET-lite node in the controller and device nodes.
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
        node.lock().unwrap().init();
        node
    }

    pub fn init(&mut self) {
        self.objects.push(NodeProfile::new());
    }

    pub fn add_object(&mut self, obj: Object) -> bool {
        self.objects.push(obj);
        self.update_node_profile();
        true
    }

    pub fn objects(&self) -> &Vec<Object> {
        return &self.objects;
    }

    pub fn find_object(&self, code: ObjectCode) -> Option<&Object> {
        for n in 0..self.objects.len() {
            if self.objects[n].code() == code {
                return Some(&self.objects[n]);
            }
        }
        None
    }

    pub fn find_object_mut(&mut self, code: ObjectCode) -> Option<&mut Object> {
        for n in 0..self.objects.len() {
            if self.objects[n].code() == code {
                return Some(&mut self.objects[n]);
            }
        }
        None
    }

    pub fn node_profile_object(&mut self) -> Option<&mut Object> {
        self.find_object_mut(NODE_PROFILE_OBJECT_CODE)
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        self.transport_mgr.add_observer(observer.clone())
    }

    pub fn send_message(&mut self, to_addr: SocketAddr, msg: &mut Message) -> bool {
        self.update_message_header(msg);
        self.transport_mgr.send(to_addr, msg)
    }

    pub fn post_message(&mut self, to_addr: SocketAddr, msg: &mut Message) -> Receiver<Message> {
        self.update_message_header(msg);
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
        msg.set_seoj(NODE_PROFILE_OBJECT_CODE);
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

    fn send_post_reopnse(&mut self, msg: Message) {
        match self.post_sender.send(msg) {
            Ok(()) => {}
            Err(_err) => {
                // warn!("{}", err);
            }
        }
        let (tx, _): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        self.post_sender = tx;
    }

    fn update_node_profile(&mut self) {
        let mut obj_codes = Vec::new();
        for obj in self.objects() {
            obj_codes.push(obj.code());
        }
        let node_prof_obj = self.node_profile_object();
        if node_prof_obj.is_none() {
            return;
        }
        let node_prof_obj = node_prof_obj.unwrap();
        let mut node_prof = NodeProfile::from(node_prof_obj);
        node_prof.update(&obj_codes);
    }
}

impl Observer for Arc<Mutex<LocalNode>> {
    fn message_received(&mut self, msg: &Message) {
        let mut node = self.lock().unwrap();
        if node.is_last_message_response(msg) {
            node.send_post_reopnse(msg.clone());
        }
    }
}
