// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::sync::Arc;
use std::sync::Mutex;

use crate::uecho::protocol::message::Message;

pub trait Observer {
    fn on_notify(&mut self, msg: &Message) -> bool;
}

pub type ObserverEntity = Box<dyn Observer + Send>;
pub type Observers = Vec<Arc<Mutex<ObserverEntity>>>;

pub fn observers_new() -> Observers {
    Vec::new()
}
