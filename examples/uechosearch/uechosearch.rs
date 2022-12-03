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

use std::env;
use std::io::Error;
use std::string::String;
use std::time::Duration;
use std::{thread, time};

use cybergarage::log::DefaultLogger;
use echonet::protocol::{Esv, Message, Property};
use echonet::util::Bytes;
use echonet::{Controller, ManufactureCode, StandardDatabase};

fn main() -> Result<(), Error> {
    for arg in env::args() {
        print!("{}", arg);
        match arg.as_str() {
            "-v" => {
                DefaultLogger::init();
            }
            &_ => {}
        }
    }

    let mut ctrl = Controller::new();
    ctrl.start();
    ctrl.search();

    thread::sleep(time::Duration::from_secs(2));

    for (i, node) in ctrl.nodes().iter().enumerate() {
        // Makes a manufacture code read (Esv::ReadRequest) message.
        let mut msg = Message::new();
        msg.set_esv(Esv::ReadRequest);
        msg.set_deoj(0x0EF001);
        let mut prop = Property::new();
        prop.set_code(0x8A);
        msg.add_property(prop);

        let mut manufacture_name = String::from("");
        let rx = ctrl.post_message(&node, &mut msg);
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => {
                let props = msg.properties();
                if 0 < props.len() {
                    let manufacture_code = Bytes::to_u32(props[0].data()) as ManufactureCode;
                    let std_db = StandardDatabase::shared();
                    let found_manufacture = std_db.find_manufacture(manufacture_code);
                    if found_manufacture.is_some() {
                        manufacture_name = found_manufacture.unwrap().name().to_string();
                    }
                }
            }
            Err(_e) => {}
        };

        println!("[{}] {} ({})", i, node.addr(), manufacture_name);

        // Prints all mandatory properties in the object.

        for (j, obj) in node.objects().iter().enumerate() {
            println!("    [{}] {:06X} ({})", j, obj.code(), obj.class_name());
            for obj_prop in obj.properties() {
                if !obj_prop.is_read_required() {
                    continue;
                }

                // Makes a property value read (Esv::ReadRequest) message.
                let mut msg = Message::new();
                msg.set_esv(Esv::ReadRequest);
                msg.set_deoj(obj.code());
                let mut prop = Property::new();
                prop.set_code(obj_prop.code());
                msg.add_property(prop);

                let mut prop_data = String::from("");
                let rx = ctrl.post_message(&node, &mut msg);
                match rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(msg) => {
                        if 0 < msg.opc() {
                            prop_data = hex::encode(msg.property(0).data());
                        }
                    }
                    Err(e) => {
                        prop_data = format!("{}", e);
                    }
                };
                println!(
                    "        [{:02X}] {}: {}",
                    obj_prop.code(),
                    obj_prop.name(),
                    prop_data
                );
            }
        }
    }

    ctrl.stop();

    Ok(())
}
