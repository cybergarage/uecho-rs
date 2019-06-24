// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub const HEADER_EHD1_ECHONET: u8 = 0x10;
pub const HEADER_EHD2_FORMAT1: u8 = 0x81;
pub const FRAME_HEADER_SIZE: usize = (1 + 1 + 2);
pub const FORMAT1_HEADER_SIZE: usize = (3 + 3 + 1 + 1);
pub const FORMAT1_MIN_SIZE: usize = (FRAME_HEADER_SIZE + FORMAT1_HEADER_SIZE);
pub const FORMAT1_PROPERTY_HEADER_SIZE: usize = 2;
pub const TID_SIZE: usize = 2;
pub const TID_MAX: usize = 65535;
pub const EOJ_SIZE: usize = 3;

pub struct Message {
    tid: [u8; 2],
    seoj: [u8; 3],
    deoj: [u8; 3],
    esv: u8,
}
impl Message {
    pub fn new() -> Message {
        Message {
            tid: [0, 0],
            seoj: [0, 0, 0],
            deoj: [0, 0, 0],
            esv: 0,
        }
    }

    pub fn parse(&mut self, msg: &[u8]) -> bool {
        if msg.len() <= FRAME_HEADER_SIZE {
            return false;
        }

        if (msg[0] != HEADER_EHD1_ECHONET) || (msg[1] != HEADER_EHD2_FORMAT1) {
            return false;
        }

        self.tid = [msg[2], msg[3]];

        if msg.len() <= FORMAT1_MIN_SIZE {
            return false;
        }

        self.seoj = [msg[4], msg[5], msg[6]];
        self.deoj = [msg[7], msg[8], msg[9]];
        self.esv = msg[10];

        true
    }
}
