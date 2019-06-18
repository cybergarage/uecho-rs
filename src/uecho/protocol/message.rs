// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub struct Message {}

impl Message {
    pub fn new() -> Message {
        Message {}
    }

    pub fn parse(&self, mut buf: &[u8]) -> bool {
        false
    }
}

#[test]
fn message_parse_test() {}
