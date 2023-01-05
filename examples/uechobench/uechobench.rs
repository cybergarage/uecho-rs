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

use echonet::log::Logger;
use echonet::protocol::{Message, Property, ESV};
use echonet::Controller;

fn usages() {
    println!(
        "Usage: uechobench"
    );
    println!(" -h : Print this message");
    println!(" -a : Request all properties");
    println!(" -v : Enable debug output");
    println!(" -n : Count of Repeat");
}

fn main() -> Result<(), Error> {
    let mut only_mandatory_properties = true;
    let args: Vec<String> = env::args().collect();
    let mut n = 0;
    let mut repeat_cnt = 1;
    while n < args.len() {
        let arg = args[n].clone();
        match arg.as_str() {
            "-v" => {
                Logger::init();
            }
            "-n" => {
                n = n + 1;
                let arg = args[n].clone();
                repeat_cnt = arg.parse().unwrap();
            }
            "-a" => {
                only_mandatory_properties = false;
            }
            "-h" => {
                usages();
                return Ok(());
            }
            &_ => {}
        }
        n = n + 1;
    }

    let mut ctrl = Controller::new();
    ctrl.start();
    ctrl.search();

    thread::sleep(time::Duration::from_secs(2));

    for _ in 0..repeat_cnt {
        for (i, node) in ctrl.nodes().iter().enumerate() {
            println!("[{}] {})", i, node.addr());

            // Prints all mandatory properties in the object.

            for (j, obj) in node.objects().iter().enumerate() {
                println!("    [{}] {:06X} ({})", j, obj.code(), obj.class_name());
                for obj_prop in obj.properties() {
                    if only_mandatory_properties && !obj_prop.is_read_required() {
                        continue;
                    }

                    // Makes a property value read (ESV::ReadRequest) message.
                    let mut msg = Message::new();
                    msg.set_esv(ESV::ReadRequest);
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
    }

    ctrl.stop();

    Ok(())
}
