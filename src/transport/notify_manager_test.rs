// Copyright (C) 2022 Satoshi Konno All rights reserved.
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

#![allow(dead_code)]

use std::sync::Arc;
use std::sync::Mutex;

use crate::protocol::Message;
use crate::transport::Observer;

pub struct TestNotifyCounter {
    pub counter: Arc<Mutex<i32>>,
}

impl TestNotifyCounter {
    pub fn new(counter: Arc<Mutex<i32>>) -> TestNotifyCounter {
        TestNotifyCounter { counter: counter }
    }
}

impl Observer for TestNotifyCounter {
    fn message_received(&mut self, _msg: &Message) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }
}

#[cfg(test)]
mod tests {

    use crate::protocol::Message;
    use crate::transport::notify_manager::*;
    use crate::transport::notify_manager_test::*;

    #[test]
    fn notify_manager_add() {
        const TEST_OBSERVER_COUNT: i32 = 10;
        let counter = Arc::new(Mutex::new(0));
        let observer = TestNotifyCounter::new(counter.clone());
        let observer = Arc::new(Mutex::new(observer));
        let mut mgr = NotifytManager::new();
        assert!(mgr.start());
        for _ in 0..TEST_OBSERVER_COUNT {
            assert!(mgr.add_observer(observer.clone()));
        }
        assert!(mgr.stop());
    }

    #[test]
    fn notify_manager_received() {
        const TEST_OBSERVER_COUNT: i32 = 10;
        let counter = Arc::new(Mutex::new(0));

        let mut mgr = NotifytManager::new();
        assert!(mgr.start());

        for _ in 0..TEST_OBSERVER_COUNT {
            let observer = TestNotifyCounter::new(counter.clone());
            assert!(mgr.add_observer(Arc::new(Mutex::new(observer))));
        }

        let msg = Message::new();
        assert!(mgr.notify(&msg));
        assert_eq!(*counter.lock().unwrap(), TEST_OBSERVER_COUNT);

        assert!(mgr.stop());
    }
}
