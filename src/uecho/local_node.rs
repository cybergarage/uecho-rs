// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::server::*;

pub struct LocalNode {
    server: Server
}

impl LocalNode {
    pub fn new() -> LocalNode {
        LocalNode {
            server: Server::new()
        }
    }
}
