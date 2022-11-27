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

use std::sync::Arc;
use std::sync::Mutex;

use crate::local_node::LocalNode;
use crate::object::{Object, ObjectCode};
use crate::property::PropertyCode;
use crate::protocol::Message;
use crate::remote_node::RemoteNode;
use crate::transport::{Observer, ObserverEntity};

pub struct DeviceNode {
    node: Arc<Mutex<LocalNode>>,
    pub remote_nodes: Vec<RemoteNode>,
}

impl DeviceNode {
    pub fn new() -> Arc<Mutex<DeviceNode>> {
        DeviceNode::new_with_node(LocalNode::new())
    }

    pub fn new_with_node(node: Arc<Mutex<LocalNode>>) -> Arc<Mutex<DeviceNode>> {
        let ctrl = Arc::new(Mutex::new(DeviceNode {
            node: node,
            remote_nodes: Vec::new(),
        }));
        ctrl
    }

    pub fn add_object(&mut self, obj: Object) -> bool {
        let mut node = self.node.lock().unwrap();
        node.add_object(obj)
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        let mut node = self.node.lock().unwrap();
        node.add_observer(observer.clone())
    }

    pub fn local_node(&self) -> Arc<Mutex<LocalNode>> {
        self.node.clone()
    }

    pub fn set_property(
        &mut self,
        obj_code: ObjectCode,
        prop_code: PropertyCode,
        data: &[u8],
    ) -> bool {
        let mut node = self.node.lock().unwrap();
        let obj = node.find_object_mut(obj_code);
        if obj.is_none() {
            return false;
        }
        let prop = obj.unwrap().find_property_mut(prop_code);
        if prop.is_none() {
            return false;
        }
        prop.unwrap().set_data(data);
        true
    }

    /// Gets the specified property data if the device node has the property, otherwise return none.
    pub fn property(&self, obj_code: ObjectCode, prop_code: PropertyCode) -> Option<Vec<u8>> {
        let node = self.node.lock().unwrap();
        let obj = node.find_object(obj_code);
        if obj.is_none() {
            return None;
        }
        let prop = obj.unwrap().find_property(prop_code);
        if prop.is_none() {
            return None;
        }
        Some(prop.unwrap().data().clone())
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

impl Observer for Arc<Mutex<DeviceNode>> {
    fn message_received(&mut self, _msg: &Message) {}
}
