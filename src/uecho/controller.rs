// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::sync::Arc;
use std::sync::Mutex;

use crate::uecho::local_node::*;
use crate::uecho::message::*;
use crate::uecho::node_profile::*;
use crate::uecho::object::*;
use crate::uecho::object::*;
use crate::uecho::protocol::esv::*;
use crate::uecho::protocol::message::*;
use crate::uecho::transport::observer::*;

pub struct Controller {
    node: LocalNode,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            node: LocalNode::new(),
        }
    }

    pub fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        self.node.add_observer(observer.clone())
    }

    pub fn search_object(&mut self, obj_code: ObjectCode) -> bool {
        let mut msg = message_serarch_new();
        msg.set_destination_object_code(obj_code);
        self.node.notify(&msg)
    }

    pub fn search_all(&mut self) -> bool {
        self.search_object(NodeProfileObject)
    }

    pub fn start(&mut self) -> bool {
        if !self.node.start() {
            return false;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        if !self.node.stop() {
            return false;
        }
        true
    }
}

impl Observer for Controller {
    fn on_notify(&mut self, _msg: &Message) {}
}
