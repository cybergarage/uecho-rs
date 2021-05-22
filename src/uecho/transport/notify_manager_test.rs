// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::protocol::message::Message;
    use crate::uecho::transport::notify_manager::*;
    use crate::uecho::transport::observer::Observer;

    struct NotifytCounter {
        pub count: i32,
    }

    impl NotifytCounter {
        pub fn new() -> NotifytCounter {
            NotifytCounter { count: 0 }
        }
    }

    impl Observer for NotifytCounter {
        fn on_notify(&mut self, msg: &Message) -> bool {
            self.count = self.count + 1;
            true
        }
    }

    #[test]
    fn notify_manager_test() {
        const TEST_OBSERVER_COUNT: i32 = 10;

        let mut mgr = NotifytManager::new();
        assert!(mgr.start());

        for _ in 1..TEST_OBSERVER_COUNT {
            let observer = NotifytCounter::new();
            assert!(mgr.add_observer(Box::new(observer)));
        }

        let msg = Message::new();

        for _ in 1..TEST_OBSERVER_COUNT {
            assert!(mgr.notify(&msg));
        }

        //assert_eq!(a, b);

        assert!(mgr.stop());
    }
}
