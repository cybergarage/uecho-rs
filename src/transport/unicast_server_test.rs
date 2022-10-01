// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;
    use std::time::Duration;

    use crate::protocol::esv::Esv;
    use crate::protocol::message::*;
    use crate::transport::interface::*;
    use crate::transport::unicast_server::*;

    use crate::log::logger;
    use crate::transport::notify_manager_test::*;

    #[test]
    fn unicast_server_test() {
        logger::init();

        fn test_udp_server(ifaddr: IpAddr) {
            logger::init();

            const TEST_OBSERVER_COUNT: i32 = 10;
            let counter = Arc::new(Mutex::new(0));

            let mut server = UnicastServer::new();

            let observer = TestNotifyCounter::new(counter.clone());
            assert!(server.add_observer(Arc::new(Mutex::new(observer))));

            assert!(server.bind(ifaddr));
            assert!(server.start());
            thread::sleep(Duration::from_secs(1));

            let mut msg = Message::new();
            msg.set_esv(Esv::ReadRequest);
            for _ in 0..TEST_OBSERVER_COUNT {
                let server_addr = server.local_addr();
                assert!(server_addr.is_ok());
                assert!(server.send(server_addr.unwrap(), &msg));
            }

            let wait_time = (TEST_OBSERVER_COUNT as u64) / 2;
            thread::sleep(Duration::from_secs(wait_time));
            assert_eq!(*counter.lock().unwrap(), TEST_OBSERVER_COUNT);

            assert!(server.stop());
        }

        for ifaddr in get_v4_interfaces() {
            test_udp_server(ifaddr)
        }
    }
}
