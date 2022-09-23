// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::local_node::LocalNode;
use crate::protocol::message::Message;
use crate::transport::observer::Observer;

pub struct LocalNodeObserver {}

impl LocalNodeObserver {
    pub fn new() -> LocalNodeObserver {
        LocalNodeObserver {}
    }
}

impl Observer for LocalNodeObserver {
    fn message_received(&mut self, _msg: &Message) {}
}
