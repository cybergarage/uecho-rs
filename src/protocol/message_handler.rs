// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::protocol::message::Message;

pub trait MessageHandler {
    fn message_received(&self, msg: &Message) -> Result<Message, String>;
}
