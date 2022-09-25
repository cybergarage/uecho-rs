// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::node_profile::*;
use crate::protocol::esv::*;
use crate::protocol::message::Message;
use crate::protocol::property::Property;

pub fn message_serarch_new() -> Message {
    let mut msg = Message::new();

    msg.set_esv(Esv::ReadRequest);
    msg.set_seoj(NODE_PROFILE_OBJECT_CODE);
    msg.set_deoj(NODE_PROFILE_OBJECT_CODE);

    let mut prop = Property::new();
    prop.set_code(NODE_PROFILE_CLASS_SELF_NODE_INSTANCE_LIST_S);
    msg.add_property(prop);

    msg
}
