// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub struct Message {}

pub const HEADER_EHD1_ECHONET: u8 = 0x10;
pub const HEADER_EHD2_FORMAT1: u8 = 0x81;

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
