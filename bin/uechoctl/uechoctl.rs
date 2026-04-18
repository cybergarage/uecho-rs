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

use std::net::IpAddr;
use std::time::Duration;
use std::{thread, time};

use clap::{Parser, Subcommand};
use echonet::Controller;
use echonet::log::Logger;
use echonet::protocol::{ESV, Message, Property};
use echonet::util::Bytes;
use echonet::{ManufactureCode, StandardDatabase};

#[derive(Parser)]
#[command(author, version, about = "ECHONET Lite controller utility")]
struct Cli {
    #[arg(short, long, global = true, help = "Enable debug output")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Search ECHONET Lite nodes")]
    Scan {
        #[arg(short, long, help = "Request all properties")]
        all: bool,
    },

    #[command(about = "Send a message to a remote node")]
    Set {
        #[arg(value_name = "IP address")]
        node_addr: IpAddr,

        #[arg(value_name = "Object code (hex)")]
        object_code: String,

        #[arg(value_name = "ESV (hex)")]
        esv: String,

        #[arg(value_name = "EPC/EDT (hex)", required = true, num_args = 1..)]
        properties: Vec<String>,
    },
}

fn parse_hex(name: &str, value: &str) -> Result<Vec<u8>, String> {
    let bytes = hex::decode(value).map_err(|_| format!("{} error: {}", name, value))?;
    if bytes.is_empty() {
        return Err(format!("{} error: {}", name, value));
    }
    Ok(bytes)
}

fn parse_esv(value: &str) -> Result<ESV, String> {
    let bytes = parse_hex("ESV", value)?;
    let esv = ESV::from_u8(bytes[0]);
    if esv == ESV::Unknown {
        return Err(format!("ESV error: {}", value));
    }
    Ok(esv)
}

fn parse_properties(values: &[String]) -> Result<(Vec<u8>, Vec<Vec<u8>>), String> {
    let mut epcs = Vec::new();
    let mut edts = Vec::new();

    for value in values {
        if epcs.len() == edts.len() {
            let epc = parse_hex("EPC", value)?;
            epcs.push(epc[0]);
            continue;
        }

        let edt = parse_hex("EDT", value)?;
        edts.push(edt);
    }

    if epcs.is_empty() {
        return Err("EPC is missing".to_string());
    }

    Ok((epcs, edts))
}

fn run_set(
    node_addr: IpAddr,
    object_code: String,
    esv: String,
    properties: Vec<String>,
) -> Result<(), String> {
    let obj_code = Bytes::to_u32(&parse_hex("Object code", &object_code)?);
    if obj_code == 0 {
        return Err("Object code is missing".to_string());
    }

    let esv = parse_esv(&esv)?;
    let (epcs, edts) = parse_properties(&properties)?;

    // Starts a new controller.

    let mut ctrl = Controller::new();
    ctrl.start();
    ctrl.search();

    thread::sleep(time::Duration::from_secs(2));

    // Genarates the specified message.

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

    // Posts the specified message to the specified controller.

    for node in ctrl.nodes().iter() {
        if node.addr().ip() != node_addr {
            continue;
        }

        match req_msg.esv() {
            ESV::WriteRequest | ESV::NotificationRequest => {
                ctrl.send_message(&node, &mut req_msg);
            }
            ESV::WriteRequestResponseRequired | ESV::ReadRequest | ESV::WriteReadRequest => {
                let rx = ctrl.post_message(&node, &mut req_msg);
                match rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(res_msg) => {
                        println!("{}", res_msg);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                };
            }
            _ => {
                eprintln!("ESV ({:X}) is not request", req_msg.esv() as u8);
            }
        }

        ctrl.stop();
        return Ok(());
    }

    eprintln!("Remote node ({}) is not found", node_addr);

    ctrl.stop();
    Ok(())
}

fn run_scan(all: bool) -> Result<(), String> {
    let only_mandatory_properties = !all;

    let mut ctrl = Controller::new();
    ctrl.start();
    ctrl.search();

    thread::sleep(time::Duration::from_secs(2));

    for (i, node) in ctrl.nodes().iter().enumerate() {
        // Makes a manufacture code read (ESV::ReadRequest) message.
        let mut msg = Message::new();
        msg.set_esv(ESV::ReadRequest);
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

    ctrl.stop();

    Ok(())
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    if cli.verbose {
        Logger::init();
    }

    match cli.command {
        Commands::Scan { all } => run_scan(all),
        Commands::Set {
            node_addr,
            object_code,
            esv,
            properties,
        } => run_set(node_addr, object_code, esv, properties),
    }
}
