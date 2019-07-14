// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::message::*;
//use crate::uecho::protocol::property::*;

pub struct Object {
    seoj: [u8; 3],
    deoj: [u8; 3],
    esv: u8,
}

impl Object {
    pub fn new() -> Object {
        Object {
            seoj: [0, 0, 0],
            deoj: [0, 0, 0],
            esv: 0,
        }
    }

    pub fn new_with(msg: &Message) -> Object {
        Object {
            seoj: [0, 0, 0],
            deoj: [0, 0, 0],
            esv: 0,
        }
    }
}
