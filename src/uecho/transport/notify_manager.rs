// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::Message;
use crate::uecho::transport::observer::*;

pub trait NotifytManager {
    fn observers(&mut self) -> &Observers;
    fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        self.observers().lock().unwrap().push(observer.clone());
        true
    }

    fn notify(&mut self, msg: &Message) -> bool {
        for (_, observer) in self.observers().lock().unwrap().iter().enumerate() {
            observer.lock().unwrap().on_notify(msg)
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
            observers: observers_new(),
        }
    }
}

impl NotifytManager for DefaultNotifytManager {
    fn observers(&mut self) -> &Observers {
        &self.observers
    }
}
