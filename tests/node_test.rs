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

use std::time::Duration;
use std::{thread, time};

use echonet::log::Logger;
use echonet::protocol::{Esv, Message, Property};
use echonet::util::Bytes;
use echonet::Node;

mod test;

#[test]
fn node() {
    Logger::init();

    let node = Node::new();

    let mut dev = test::TestDevice::new(node.clone());
    assert!(dev.lock().unwrap().start());
    let dev_obj_code = dev.lock().unwrap().code();

    let mut ctrl = test::TestController::new(node.clone());
    assert!(ctrl.start());
    assert!(ctrl.search());

    thread::sleep(time::Duration::from_secs(2));

    let mut found_local_node = false;
    for remote_node in ctrl.nodes() {
        if node.lock().unwrap().has_interface(remote_node.addr().ip()) {
            found_local_node = true;
        }
    }

    assert!(found_local_node);

    for remote_node in ctrl.nodes() {
        if !node.lock().unwrap().has_interface(remote_node.addr().ip()) {
            continue;
        }

        let req_stats = vec![0x30 as u8, 0x31];
        let res_stats = vec![0x30 as u8, 0x31];

        for (n, req_stat) in req_stats.iter().enumerate() {
            // Writes a property value (Esv::WriteRequestResponseRequired).

            let mut req_msg = Message::new();
            req_msg.set_deoj(dev_obj_code);
            req_msg.set_esv(Esv::WriteRequestResponseRequired);
            let mut prop = Property::new();
            prop.set_code(0x80);
            prop.set_data(vec![*req_stat]);
            req_msg.add_property(prop);

            let rx = ctrl.post_message(&remote_node, &mut req_msg);
            match rx.recv_timeout(Duration::from_secs(5)) {
                Ok(res_meg) => {
                    assert_eq!(res_meg.esv(), Esv::WriteResponse);
                    assert_eq!(res_meg.opc(), 1);
                }
                Err(e) => {
                    panic!("{}", e);
                }
            };

            // Reads a property value (Esv::ReadRequest).

            let mut req_msg = Message::new();
            req_msg.set_deoj(dev_obj_code);
            req_msg.set_esv(Esv::ReadRequest);
            let mut prop = Property::new();
            prop.set_code(0x80);
            req_msg.add_property(prop);

            let rx = ctrl.post_message(&remote_node, &mut req_msg);
            match rx.recv_timeout(Duration::from_secs(5)) {
                Ok(res_meg) => {
                    assert_eq!(res_meg.esv(), Esv::ReadResponse);
                    assert_eq!(res_meg.opc(), 1);
                    let prop = res_meg.property(0);
                    assert_eq!(Bytes::to_u32(prop.data()), res_stats[n] as u32);
                }
                Err(e) => {
                    panic!("{}", e);
                }
            };
        }
    }

    assert!(dev.lock().unwrap().stop());
    assert!(ctrl.stop());
}
