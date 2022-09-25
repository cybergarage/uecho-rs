// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::time::Duration;
use std::{thread, time};

use uecho::controller::Controller;
use uecho::log::*;
use uecho::property::Property;
use uecho::protocol::esv::Esv;
use uecho::protocol::message::Message;

fn main() {
    logger::init();

    let mut ctrl = Controller::new();
    ctrl.start();

    thread::sleep(time::Duration::from_secs(2));

    for (i, node) in ctrl.nodes().iter().enumerate() {
        println!("[{}] {}", i, node.addr());
        for (j, obj) in node.objects().iter().enumerate() {
            println!("    [{}] {}", j, obj.code());
            for obj_prop in obj.properties() {
                let mut msg = Message::new();
                msg.set_esv(Esv::ReadRequest);
                msg.set_deoj(obj.code());
                let mut prop = Property::new();
                prop.set_code(obj_prop.code());
                let rx = ctrl.post_message(&node, &mut msg);
                match rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(msg) => {
                        println!("        [{} {}]", prop.code(), hex::encode(msg.bytes()));
                    }
                    Err(_e) => {
                        println!("        [{}]", prop.code());
                    }
                };
            }
        }
    }

    ctrl.search_all();
    ctrl.stop();
}
