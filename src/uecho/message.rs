// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::node_profile::*;
use crate::uecho::protocol::esv::*;
use crate::uecho::protocol::message::Message;
use crate::uecho::protocol::property::Property;

pub fn message_serarch_new() -> Message {
    let mut msg = Message::new();

    msg.set_esv(Esv::ReadRequest);
    msg.set_source_object_code(NodeProfileObject);
    msg.set_destination_object_code(NodeProfileObject);

    let mut prop = Property::new();
    prop.set_code(NodeProfileClassSelfNodeInstanceListS);
    msg.add_property(prop);

    msg
}
