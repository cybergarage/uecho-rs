// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::observer::{ObserverEntity, Observers};

pub trait NotifytManager {
    fn observers(&mut self) -> &Observers;
    fn add_observer(&mut self, observer: ObserverEntity) -> bool;

    fn notify(&mut self, msg: &Message) -> bool {
        for (_, observer) in self.observers().iter().enumerate() {
            if !observer.lock().unwrap().on_notify(msg) {
                return false;
            }
        }
        true
    }

    fn start(&mut self) -> bool {
        true
    }

    fn stop(&mut self) -> bool {
        true
    }
}

pub struct DefaultNotifytManager {
    observers: Observers,
}

impl DefaultNotifytManager {
    pub fn new() -> DefaultNotifytManager {
        DefaultNotifytManager {
            observers: Vec::new(),
        }
    }
}

impl NotifytManager for DefaultNotifytManager {
    fn observers(&mut self) -> &Observers {
        &self.observers
    }

    fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        self.observers.push(Arc::new(Mutex::new(observer)));
        true
    }
}
