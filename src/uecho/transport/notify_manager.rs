// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::cell::RefCell;

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::observer::Observer;

pub struct NotifytManager {
    observers: Vec<RefCell<Box<dyn Observer>>>,
}

impl NotifytManager {
    pub fn new() -> NotifytManager {
        NotifytManager {
            observers: Vec::new(),
        }
    }

    pub fn notify(&mut self, msg: &Message) -> bool {
        for (_, observer) in self.observers.iter().enumerate() {
            if !observer.borrow_mut().on_notify(msg) {
                return false;
            }
        }
        true
    }

    pub fn add_observer(&mut self, observer: Box<dyn Observer>) -> bool {
        self.observers.push(RefCell::new(observer));
        true
    }

    pub fn start(&mut self) -> bool {
        true
    }

    pub fn stop(&mut self) -> bool {
        true
    }
}
