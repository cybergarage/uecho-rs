// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::protocol::message::Message;
use crate::transport::observer::*;

pub trait NotifytManager {
    fn observers(&mut self) -> &mut Observers;
    fn add_observer(&mut self, observer: ObserverEntity) -> bool {
        self.observers().push(observer.clone());
        true
    }

    fn notify(&mut self, msg: &Message) -> bool {
        for (_, observer) in self.observers().iter().enumerate() {
            observer.lock().unwrap().message_received(msg)
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
    fn observers(&mut self) -> &mut Observers {
        &mut self.observers
    }
}
