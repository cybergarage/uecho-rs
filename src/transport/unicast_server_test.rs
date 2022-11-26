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

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;
    use std::time;

    use crate::protocol::Esv;
    use crate::protocol::Message;
    use crate::transport::interface::*;
    use crate::transport::unicast_server::*;

    use crate::log::logger;
    use crate::transport::notify_manager_test::*;

    #[test]
    fn unicast_server() {
        logger::init();

        fn test_udp_server(ifaddr: IpAddr) {
            logger::init();

            const TEST_OBSERVER_COUNT: i32 = 1;
            let counter = Arc::new(Mutex::new(0));

            let mut server = UnicastServer::new();

            let observer = TestNotifyCounter::new(counter.clone());
            assert!(server.add_observer(Arc::new(Mutex::new(observer))));

            assert!(server.bind(ifaddr));
            assert!(server.start());
            thread::sleep(time::Duration::from_secs(1));

            let mut msg = Message::new();
            msg.set_esv(Esv::ReadRequest);
            for _ in 0..TEST_OBSERVER_COUNT {
                let server_addr = server.local_addr();
                assert!(server_addr.is_ok());
                assert!(server.send(server_addr.unwrap(), &msg));
                thread::sleep(time::Duration::from_secs(1));
            }

            let wait_time = (TEST_OBSERVER_COUNT as u64) / 5;
            assert_eq!(*counter.lock().unwrap(), TEST_OBSERVER_COUNT);

            assert!(server.stop());
        }

        for ifaddr in get_v4_interfaces() {
            test_udp_server(ifaddr);
        }
    }
}
