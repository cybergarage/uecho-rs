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
