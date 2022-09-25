// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use uecho::controller::Controller;
use uecho::log::*;
use uecho::property::Property;
use uecho::protocol::message::Message;

fn main() {
    logger::init();

    let mut ctrl = Controller::new();
    ctrl.start();

    for node in ctrl.nodes() {
        for obj in node.objects() {
            for obj_prop in obj.properties() {
                let mut msg = Message::new();
                msg.set_destination_object_code(obj.code());
                let mut prop = Property::new();
                prop.set_code(obj_prop.code());
                ctrl.send_message(node, &mut msg);
            }
        }
    }

    ctrl.search_all();
    ctrl.stop();
}
