// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec::IntoIter;

use crate::controller_observer::ControllerObserver;
use crate::local_node::*;
use crate::message::*;
use crate::node_profile::*;
use crate::object::*;
use crate::protocol::esv::*;
use crate::protocol::message::*;
use crate::remote_node::*;
use crate::transport::default::PORT;
use crate::transport::observer::*;

pub struct Controller {
    observer: Arc<Mutex<ControllerObserver>>,
    pub remote_nodes: Vec<RemoteNode>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            observer: ControllerObserver::new(),
            remote_nodes: Vec::new(),
        }
    }

    pub fn nodes(&self) -> &Vec<RemoteNode> {
        // TODO: Return remote nodes in the observer
        return &self.remote_nodes;
    }

    pub fn search_object(&mut self, obj_code: ObjectCode) -> bool {
        let mut ctrl = self.observer.lock().unwrap();
        ctrl.search_object(obj_code)
    }

    pub fn search_all(&mut self) -> bool {
        self.search_object(NODE_PROFILE_OBJECT_CODE)
    }

    pub fn send_message(&self, remote_node: &RemoteNode, msg: &Message) -> bool {
        let ctrl = self.observer.lock().unwrap();
        ctrl.send_message(remote_node, msg)
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
