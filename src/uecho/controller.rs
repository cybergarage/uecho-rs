// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::local_node::*;

pub struct Controller {
    node: LocalNode
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            node: LocalNode::new()
        }
    }
}
