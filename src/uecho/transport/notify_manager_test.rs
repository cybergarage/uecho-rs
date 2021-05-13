// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::protocol::message::Message;
    use crate::uecho::transport::notify_manager::*;

    struct NotifytCounter {
        count: i32,
    }

    impl NotifytCounter {
        pub fn new() -> NotifytCounter {
            NotifytCounter { count: 1 }
        }

        pub fn on_notify(&mut self, msg: &Message) -> bool {
            self.count = self.count + 1;
            true
        }
    }

    #[test]
    fn notify_manager_test() {
        let mut mgr = NotifytManager::new();
        assert!(mgr.start());

        let msg = Message::new();

        assert!(mgr.notify(&msg));

        assert!(mgr.stop());
    }
}
