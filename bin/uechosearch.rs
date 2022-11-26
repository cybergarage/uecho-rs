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

use echonet::log::*;
use echonet::protocol::Esv;
use echonet::protocol::Message;
use echonet::protocol::Property;
use echonet::Controller;

fn main() {
    // logger::init();

    let mut ctrl = Controller::new();
    ctrl.start();
    ctrl.search();

    thread::sleep(time::Duration::from_secs(2));

    for (i, node) in ctrl.nodes().iter().enumerate() {
        println!("[{}] {}", i, node.addr());
        for (j, obj) in node.objects().iter().enumerate() {
            println!("    [{}] {:06X} ({})", j, obj.code(), obj.class_name());
            for obj_prop in obj.properties() {
                if !obj_prop.is_read_required() {
                    continue;
                }

                let mut msg = Message::new();
                msg.set_esv(Esv::ReadRequest);
                msg.set_deoj(obj.code());
                let mut prop = Property::new();
                prop.set_code(obj_prop.code());
                msg.add_property(prop);

                let rx = ctrl.post_message(&node, &mut msg);
                print!("        [{:02X}] {}: ", obj_prop.code(), obj_prop.name());
                match rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(msg) => {
                        for msg_prop in msg.properties() {
                            print!("{}", hex::encode(msg_prop.data()));
                        }
                        println!("");
                    }
                    Err(_e) => {
                        println!("{}", "<timeout>");
                    }
                };
            }
        }
    }

    ctrl.stop();
}
