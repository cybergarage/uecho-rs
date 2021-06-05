// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::cell::RefCell;

use crate::uecho::protocol::message::Message;

pub trait Observer {
    fn on_notify(&mut self, msg: &Message) -> bool;
}

pub type Observers = RefCell<Vec<RefCell<Box<dyn Observer>>>>;

// NOTE: Could not override new() for alias type
// trait ObserverMethods {
//     fn new() -> Observers;
// }
//
// impl ObserverMethods for Observers {
//     fn new() -> Observers {
//         RefCell::new(Vec::new())
//     }
// }

pub fn observer_new() -> Observers {
    RefCell::new(Vec::new())
}
