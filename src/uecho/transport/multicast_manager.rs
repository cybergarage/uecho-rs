// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::transport::multicast_server::MulticastServer;

pub struct MulticastManager {
    mcast_server: MulticastServer,
}

impl MulticastManager {
    pub fn new() -> MulticastManager {
        MulticastManager {
            mcast_server: MulticastServer::new(),
        }
    }

    pub fn start(&mut self) -> bool {
        self.mcast_server.start()
    }

    pub fn stop(&mut self) -> bool {
        self.mcast_server.stop()
    }
}
