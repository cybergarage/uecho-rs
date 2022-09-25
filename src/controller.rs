// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;

use crate::controller_observer::ControllerObserver;
use crate::node_profile::*;
use crate::object::*;
use crate::protocol::message::*;
use crate::remote_node::*;

pub struct Controller {
    observer: Arc<Mutex<ControllerObserver>>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            observer: ControllerObserver::new(),
        }
    }

    pub fn nodes(&mut self) -> Vec<RemoteNode> {
        // TODO: Return remote nodes in the observer directly
        let mut nodes = Vec::new();
        let ctrl = self.observer.lock().unwrap();
        for node in ctrl.nodes() {
            nodes.push(node.clone());
        }
        nodes
    }

    pub fn search_object(&mut self, obj_code: ObjectCode) -> bool {
        let mut ctrl = self.observer.lock().unwrap();
        ctrl.search_object(obj_code)
    }

    pub fn search_all(&mut self) -> bool {
        self.search_object(NODE_PROFILE_OBJECT_CODE)
    }

    pub fn send_message(&self, remote_node: &RemoteNode, msg: &mut Message) -> bool {
        let ctrl = self.observer.lock().unwrap();
        ctrl.send_message(remote_node, msg)
    }

    pub fn post_message(&self, remote_node: &RemoteNode, msg: &mut Message) -> Receiver<Message> {
        let ctrl = self.observer.lock().unwrap();
        ctrl.post_message(remote_node, msg)
    }

    pub fn start(&mut self) -> bool {
        let mut ctrl = self.observer.lock().unwrap();
        ctrl.start()
    }

    pub fn stop(&mut self) -> bool {
        let mut ctrl = self.observer.lock().unwrap();
        ctrl.stop()
    }
}
