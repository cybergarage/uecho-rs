// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::local_node::*;
use crate::uecho::protocol::esv::*;
use crate::uecho::protocol::message::*;

pub struct Controller {
    node: LocalNode,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            node: LocalNode::new(),
        }
    }

    pub fn searchwithesv(&mut self, esv: Esv) -> bool {
        true
    }

    pub fn search(&mut self) -> bool {
        self.searchwithesv(Esv::ReadRequest)
    }

    pub fn on_notify(&mut self, _msg: &Message) -> bool {
        true
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
