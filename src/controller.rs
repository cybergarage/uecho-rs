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

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;

use crate::controller_observer::ControllerObserver;
use crate::local_node::LocalNode;
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

    pub fn new_with_node(node: Arc<Mutex<LocalNode>>) -> Controller {
        Controller {
            observer: ControllerObserver::new_with_node(node),
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
        ctrl.start();
        ctrl.add_observer(Arc::new(Mutex::new(self.observer.clone())));

        let local_node = ctrl.local_node();
        local_node
            .lock()
            .unwrap()
            .add_observer(Arc::new(Mutex::new(local_node.clone())));

        true
    }

    pub fn stop(&mut self) -> bool {
        let mut ctrl = self.observer.lock().unwrap();
        ctrl.stop()
    }
}
