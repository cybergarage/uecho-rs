// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub struct Message {}

impl Message {
    pub fn new() -> Message {
        Message {}
    }

    pub fn parse(&self, buf: &[u8]) -> bool {
        if buf.len() <= 0 {
            return false;
        }
        true
    }
}

#[test]
fn message_parse_test() {
    let test_msg_bytes = &[
        //EHD1Echonet,
        //EHD2Format1,
        0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
        0xF0,
        //ESVNotification,
        //byte(opc),
        //1, 1, 'a',
        //2, 2, 'b', 'c',
        //3, 3, 'c', 'd', 'e'
    ];
    let msg = Message::new();
    assert!(msg.parse(test_msg_bytes));
}
