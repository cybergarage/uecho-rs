// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::protocol::property::*;

pub const HEADER_EHD1_ECHONET: u8 = 0x10;
pub const HEADER_EHD2_FORMAT1: u8 = 0x81;
pub const FRAME_HEADER_SIZE: usize = (1 + 1 + 2);
pub const FORMAT1_HEADER_SIZE: usize = (3 + 3 + 1 + 1);
pub const TID_MAX: usize = 65535;

pub struct Message {
    tid: [u8; 2],
    seoj: [u8; 3],
    deoj: [u8; 3],
    esv: u8,
    opc: usize,
}
impl Message {
    pub fn new() -> Message {
        Message {
            tid: [0, 0],
            seoj: [0, 0, 0],
            deoj: [0, 0, 0],
            esv: 0,
            opc: 0,
        }
    }

    fn to_object_code(&self, eoj: &[u8]) -> u32 {
        ((eoj[0] as u32) << 16) + ((eoj[1] as u32) << 8) + (eoj[2] as u32)
    }

    pub fn source_object_code(&self) -> u32 {
        self.to_object_code(&self.seoj)
    }

    pub fn destination_object_code(&self) -> u32 {
        self.to_object_code(&self.deoj)
    }

    pub fn opc(&self) -> usize {
        return self.opc as usize;
    }

    pub fn parse(&mut self, msg: &[u8]) -> bool {
        // Parser ECHONET Lite Header (EHD)

        let header = &msg[0..];

        if header.len() <= FRAME_HEADER_SIZE {
            return false;
        }

        self.tid = [header[2], header[3]];

        if (header[0] != HEADER_EHD1_ECHONET) || (header[1] != HEADER_EHD2_FORMAT1) {
            return false;
        }

        // Parse ECHONET Lite Data (EDATA)

        let edata = &msg[4..];

        if edata.len() <= FORMAT1_HEADER_SIZE {
            return false;
        }

        self.seoj = [edata[0], edata[1], edata[2]];
        self.deoj = [edata[3], edata[4], edata[5]];
        self.esv = edata[6];
        self.opc = edata[7] as usize;

        let mut prop_msg_offset = 8;
        for _n in 0..self.opc {
            let prop_msg = &msg[prop_msg_offset..];
            let mut prop = Property::new();
            if !prop.parse(prop_msg) {
                return false
            }
            prop_msg_offset += prop.size();
        }

        true
    }
}
