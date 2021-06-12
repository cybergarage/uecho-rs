// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;
    use std::time::Duration;

    use crate::uecho::protocol::esv::*;
    use crate::uecho::protocol::message::*;
    use crate::uecho::transport::unicast_udp_server::*;

    use crate::uecho::transport::notify_manager_test::*;

    #[test]
    fn unicast_udp_server_test() {
        const TEST_OBSERVER_COUNT: i32 = 10;
        let counter = Arc::new(Mutex::new(0));

        let mut server = UnicastUdpServer::new();

        for _ in 0..TEST_OBSERVER_COUNT {
            let observer = TestNotifyCounter::new(counter.clone());
            //assert!(server.add_observer(Box::new(observer)));
        }

        assert!(server.start());
        thread::sleep(Duration::from_secs(1));

        let server_addr = server.local_addr();
        assert!(server_addr.is_ok());

        let mut msg = Message::new();
        msg.set_esv(ESV_READ_REQUEST);
        assert!(server.send_message(server_addr.unwrap(), &msg));

        // assert_eq!(*counter.lock().unwrap(), TEST_OBSERVER_COUNT);

        assert!(server.stop());
    }
}
