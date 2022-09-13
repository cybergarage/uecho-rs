// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::sync::Arc;
use std::sync::Mutex;

use crate::protocol::message::Message;
use crate::transport::observer::Observer;

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

    use crate::protocol::message::Message;
    use crate::transport::notify_manager::*;
    use crate::transport::notify_manager_test::*;

    #[test]
    fn notify_manager_test() {
        const TEST_OBSERVER_COUNT: i32 = 10;
        let counter = Arc::new(Mutex::new(0));

        let mut mgr = DefaultNotifytManager::new();
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
