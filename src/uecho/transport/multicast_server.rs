// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::transport::server::Server;

pub struct MulticastServer {}

impl MulticastServer {
    pub fn new() -> MulticastServer {
        MulticastServer {}
    }
}

impl Server for MulticastServer {
    fn start(&self) -> bool {
        true
    }

    fn stop(&self) -> bool {
        true
    }
}
