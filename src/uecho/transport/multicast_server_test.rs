// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;
    use std::time::Duration;

    use crate::uecho::protocol::esv::*;
    use crate::uecho::protocol::message::*;
    use crate::uecho::transport::interface::*;
    use crate::uecho::transport::multicast_server::*;

    use crate::uecho::log::logger;
    use crate::uecho::transport::notify_manager_test::*;

    #[test]
    fn multicast_server_test() {
        fn test_multicast_server(ifaddr: IpAddr) {
            logger::init();

            const TEST_OBSERVER_COUNT: i32 = 10;
            let counter = Arc::new(Mutex::new(0));

            let mut server = MulticastServer::new();

            let observer = TestNotifyCounter::new(counter.clone());
            assert!(server.add_observer(Arc::new(Mutex::new(observer))));

            assert!(server.bind(ifaddr));
            assert!(server.start());
            thread::sleep(Duration::from_secs(1));

            let mut msg = Message::new();
            msg.set_esv(ESV_READ_REQUEST);
            for _ in 0..TEST_OBSERVER_COUNT {
                let server_addr = server.local_addr();
                assert!(server_addr.is_ok());
                assert!(server.notify(&msg));
            }

            let wait_time = (TEST_OBSERVER_COUNT as u64) / 2;
            thread::sleep(Duration::from_secs(wait_time));
            assert_eq!(*counter.lock().unwrap(), TEST_OBSERVER_COUNT);

            assert!(server.stop());
        }

        for ifaddr in get_v4_interfaces() {
            test_multicast_server(ifaddr)
        }
    }
}
