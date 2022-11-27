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

use crate::controller_node::ControllerNode;
use crate::local_node::LocalNode;
use crate::node_profile::*;
use crate::object::*;
use crate::protocol::Message;
use crate::remote_node::*;

/// Controller represents a ECHONET-lite controller node to communicate other ECHONET-lite nodes.
/// # Examples
/// ```
/// use std::{thread, time};
///
/// use echonet::Controller;
/// use echonet::Property;
/// use echonet::protocol::Esv;
/// use echonet::protocol::Message;
///
/// let mut ctrl = Controller::new();
/// ctrl.start();
/// ctrl.search();
/// thread::sleep(time::Duration::from_secs(2));
/// for (i, node) in ctrl.nodes().iter().enumerate() {
///     println!("[{}] {}", i, node.addr());
///     for (j, obj) in node.objects().iter().enumerate() {
///         println!("    [{}] {:06X}", j, obj.code());
///         for obj_prop in obj.properties() {
///             if !obj_prop.is_read_required() {
///                 continue;
///             }
///             print!("        [{:02X}] {}:", obj_prop.code(), obj_prop.name());
///             let mut msg = Message::new();
///             msg.set_esv(Esv::ReadRequest);
///             msg.set_deoj(obj.code());
///             let mut prop = Property::new();
///             prop.set_code(obj_prop.code());
///             let rx = ctrl.post_message(&node, &mut msg);
///             match rx.recv_timeout(time::Duration::from_secs(1)) {
///                 Ok(msg) => {
///                     for msg_prop in msg.properties() {
///                         print!("{}", hex::encode(msg_prop.data()));
///                     }
///                     println!("");
///                 }
///                 Err(_e) => {
///                     println!("{}",  "<timeout>");
///                 }
///             };
///         }
///     }
/// }
/// ctrl.stop();
/// ```
pub struct Controller {
    node: Arc<Mutex<ControllerNode>>,
}

impl Controller {
    /// Create a new controller.
    pub fn new() -> Controller {
        Controller {
            node: ControllerNode::new(),
        }
    }

    /// Create a new controller with the node to which it belongs.
    pub fn new_with_node(node: Arc<Mutex<LocalNode>>) -> Controller {
        Controller {
            node: ControllerNode::new_with_node(node),
        }
    }

    /// Returns all searched remote nodes.
    pub fn nodes(&mut self) -> Vec<RemoteNode> {
        let mut nodes = Vec::new();
        let ctrl = self.node.lock().unwrap();
        for node in ctrl.nodes() {
            nodes.push(node.clone());
        }
        nodes
    }

    /// Searches only the specified object nodes on the local network.
    pub fn search_object(&mut self, obj_code: ObjectCode) -> bool {
        let mut ctrl = self.node.lock().unwrap();
        ctrl.search_object(obj_code)
    }

    /// Searches all ECHONET-lite nodes on the local network.
    pub fn search(&mut self) -> bool {
        self.search_object(NODE_PROFILE_OBJECT_CODE)
    }

    /// Sends the specified message to the specified remote node. The function automatically updates the SEOJ (Source ECHONET-Lite object) and TID (Transaction ID) in the specified message, so you do not need to set the message fields.
    pub fn send_message(&self, remote_node: &RemoteNode, msg: &mut Message) -> bool {
        let ctrl = self.node.lock().unwrap();
        ctrl.send_message(remote_node, msg)
    }

    /// Posts the specified message to the remote node and waits for the response using the messaging channel. TThe function automatically updates the SEOJ (Source ECHONET-Lite object) and TID (Transaction ID) in the specified message, so you do not need to set the message fields.
    pub fn post_message(&self, remote_node: &RemoteNode, msg: &mut Message) -> Receiver<Message> {
        let ctrl = self.node.lock().unwrap();
        ctrl.post_message(remote_node, msg)
    }

    /// Starts the controller node to communicate with other ECHONET-Lite nodes on the local network.
    pub fn start(&mut self) -> bool {
        let mut ctrl = self.node.lock().unwrap();
        if !ctrl.start() {
            return false;
        }
        ctrl.add_observer(Arc::new(Mutex::new(self.node.clone())));

        let local_node = ctrl.local_node();
        local_node
            .lock()
            .unwrap()
            .add_observer(Arc::new(Mutex::new(local_node.clone())));

        true
    }

    /// Stops the controller node, and clears all searched remote nodes.
    pub fn stop(&mut self) -> bool {
        let mut ctrl = self.node.lock().unwrap();
        ctrl.stop()
    }
}
