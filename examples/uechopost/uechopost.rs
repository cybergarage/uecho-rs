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
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;
use std::{thread, time};

use echonet::log::Logger;
use echonet::protocol::{Esv, Message, Property};
use echonet::util::Bytes;
use echonet::Controller;
use hex;

fn usages() {
    println!(
        "Usage: uechopost <IP address> <Object code (hex)> <ESV (hex)> (<EPC (hex)> <EDT (hex)>)?"
    );
    println!(" -h : Print this message");
    println!(" -d : Enable debug output");
}

fn main() {
    let mut program_name = String::new();
    let ipaddr_none = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let mut node_addr = ipaddr_none.clone();
    let mut esv = Esv::Unknown;
    let mut obj_code = 0;
    let mut epcs = Vec::new();
    let mut edts = Vec::new();

    // Parses specified command line arguments.

    for arg in env::args() {
        match arg.as_str() {
            "-v" => {
                Logger::init();
            }
            "-h" => {
                usages();
                return;
            }
            &_ => {
                if program_name.len() == 0 {
                    program_name = arg.clone();
                    continue;
                }
                if node_addr == ipaddr_none {
                    let arg_addr = arg.parse();
                    if arg_addr.is_err() {
                        usages();
                        eprintln!("IP address error: {}", arg);
                        return;
                    }
                    node_addr = arg_addr.unwrap();
                    continue;
                }
                if obj_code == 0 {
                    let arg_obj = hex::decode(arg.clone());
                    if arg_obj.is_err() {
                        usages();
                        eprintln!("Object code error: {}", arg);
                        return;
                    }
                    obj_code = Bytes::to_u32(&arg_obj.unwrap());
                    continue;
                }
                if esv == Esv::Unknown {
                    let arg_esv = hex::decode(arg.clone());
                    if arg_esv.is_err() {
                        usages();
                        eprintln!("ESV error: {}", arg);
                        return;
                    }
                    let arg_esv = arg_esv.unwrap();
                    esv = Esv::from_u8(arg_esv[0]);
                    if esv == Esv::Unknown {
                        usages();
                        eprintln!("ESV error: {}", arg);
                        return;
                    }
                    continue;
                }
                if epcs.len() == edts.len() {
                    let arg_epc = hex::decode(arg.clone());
                    if arg_epc.is_err() {
                        usages();
                        eprintln!("EPC error: {}", arg);
                        return;
                    }
                    let arg_epc = arg_epc.unwrap();
                    epcs.push(arg_epc[0] as u8);
                    continue;
                }
                if edts.len() < epcs.len() {
                    let arg_edt = hex::decode(arg.clone());
                    if arg_edt.is_err() {
                        usages();
                        eprintln!("EDT error: {}", arg);
                        return;
                    }
                    let arg_edt = arg_edt.unwrap();
                    edts.push(arg_edt);
                }
            }
        }
    }

    // Checks specified command line rguments.

    if node_addr == ipaddr_none {
        usages();
        eprintln!("IP address is missing");
        return;
    }

    if esv == Esv::Unknown {
        usages();
        eprintln!("ESV is missing");
        return;
    }

    if obj_code == 0 {
        usages();
        eprintln!("Object code is missing");
        return;
    }

    if epcs.len() == 0 {
        usages();
        eprintln!("EPC is missing");
        return;
    }

    // Starts a new controller.

    let mut ctrl = Controller::new();
    ctrl.start();
    ctrl.search();

    thread::sleep(time::Duration::from_secs(2));

    // Posts a message to the specified controller.

    for node in ctrl.nodes().iter() {
        if node.addr().ip() != node_addr {
            continue;
        }

        let mut req_msg = Message::new();
        req_msg.set_esv(esv);
        req_msg.set_deoj(obj_code);
        for (n, epc) in epcs.iter().enumerate() {
            let mut prop = Property::new();
            prop.set_code(*epc);
            if n < edts.len() {
                prop.set_data(edts[n].clone());
            }
            req_msg.add_property(prop);
        }

        let rx = ctrl.post_message(&node, &mut req_msg);
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(res_msg) => {
                println!("{}", res_msg);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };

        ctrl.stop();
        return;
    }

    eprintln!("Remote node ({}) is not found", node_addr);

    ctrl.stop();
}
