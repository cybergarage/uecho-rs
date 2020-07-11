// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::Message;

pub trait MessageHandler {
    fn message_received(msg: &Message) -> Result<Message, String>;
}
