// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::transport::observer::Observer;

pub struct NotifytManager {
    observers: Vec<Box<Observer>>,
}

impl NotifytManager {
    pub fn new() -> NotifytManager {
        NotifytManager {
            observers:Vec::new(),
        }
    }

    pub fn start(&mut self) -> bool {
        true
    }

    pub fn stop(&mut self) -> bool {
        true
    }
}
