// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::Message;

pub trait Observer {
    fn on_notify(&self, msg: &Message) -> bool;
}
