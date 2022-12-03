// Copyright (C) 2022 The uecho-rs Authors All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::protocol::Message;
use crate::transport::observer::*;

pub trait NotifytManager {
    fn observers(&mut self) -> &mut Observers;

    fn add_observer(&mut self, observer: ObserverObject) -> bool {
        // FIXME: FIX not to register the same observer
        // for reg_observer in self.observers().iter() {
        //     let new_observer = observer.lock().unwrap();
        //     let observer = reg_observer.lock().unwrap();
        //     if ::core::ptr::eq(&new_observer, &observer) {
        //         return true;
        //     }
        // }
        self.observers().push(observer);
        true
    }

    fn notify(&mut self, msg: &Message) -> bool {
        for observer in self.observers().iter() {
            let mut observer = observer.lock().unwrap();
            observer.message_received(msg);
        }
        true
    }

    fn num_observers(&mut self) -> usize {
        self.observers().len()
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
    fn observers(&mut self) -> &mut Observers {
        &mut self.observers
    }
}
