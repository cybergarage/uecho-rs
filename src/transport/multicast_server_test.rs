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
    use std::time::Duration;

    use crate::protocol::esv::Esv;
    use crate::protocol::message::*;
    use crate::transport::interface::*;
    use crate::transport::multicast_server::*;

    use crate::log::logger;
    use crate::transport::notify_manager_test::*;

    #[test]
    fn multicast_server() {
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
            msg.set_esv(Esv::Notification);
            for _ in 0..TEST_OBSERVER_COUNT {
                let server_addr = server.local_addr();
                assert!(server_addr.is_ok());
                assert!(server.notify(&msg));
            }

            let wait_time = (TEST_OBSERVER_COUNT as u64) / 5;
            thread::sleep(Duration::from_secs(wait_time));
            assert_eq!(*counter.lock().unwrap(), TEST_OBSERVER_COUNT);

            assert!(server.stop());
        }

        for ifaddr in get_v4_interfaces() {
            test_multicast_server(ifaddr);
        }
    }
}
