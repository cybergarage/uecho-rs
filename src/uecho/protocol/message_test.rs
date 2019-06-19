// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[test]
fn message_parse_test() {
    let test_msg_bytes = &[
        HEADER_EHD1_ECHONET,
        HEADER_EHD2_FORMAT1,
        0x00, 0x00,
        0xA0, 0xB0, 0xC0, 
        0xD0, 0xE0, 0xF0,
        ESV_NOTIFICATION,
        3,
        1, 1, 0x61,
        2, 2, 0x62, 0x63,
        3, 3, 0x64, 0x65, 0x66
    ];

    let msg = Message::new();
    assert!(msg.parse(test_msg_bytes));
}
