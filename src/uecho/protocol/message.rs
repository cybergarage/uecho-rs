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
    ehd: [u8; 2],
    tid: [u8; 2],
    seoj: [u8; 3],
    deoj: [u8; 3],
    esv: u8,
    opc: usize,
    properties: Vec<Property>,
}

impl Message {
    pub fn new() -> Message {
        Message {
            ehd: [0; 2],
            tid: [0; 2],
            seoj: [0; 3],
            deoj: [0; 3],
            esv: 0,
            opc: 0,
            properties: Vec::<Property>::new(),
        }
    }

    pub fn tid(&self) -> u32 {
        ((self.tid[0] as u32) << 8) + (self.tid[1] as u32)
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
        self.opc as usize
    }

    pub fn is_format1(&mut self) -> bool {
        if (self.ehd[0] != HEADER_EHD1_ECHONET) || (self.ehd[1] != HEADER_EHD2_FORMAT1) {
            return false;
        }

        true
    }

    pub fn parse(&mut self, msg: &[u8]) -> bool {
        if !self.parse_header(msg) {
            return false
        }

        if !self.is_format1(){
            return false
        }

        let format1_msg = &(*msg)[4..];
        if !self.parse_format1_data(format1_msg) {
            return false
        }

        true
    }

    fn parse_header(&mut self, header: &[u8]) -> bool {
        if header.len() <= FRAME_HEADER_SIZE {
            return false;
        }

        self.ehd = [header[0], header[1]];
        self.tid = [header[2], header[3]];

        true
    }

    fn parse_format1_data(&mut self, msg: &[u8]) -> bool {
        if msg.len() <= FORMAT1_HEADER_SIZE {
            return false;
        }

        self.seoj = [msg[0], msg[1], msg[2]];
        self.deoj = [msg[3], msg[4], msg[5]];
        self.esv = msg[6];
        self.opc = msg[7] as usize;

        let mut prop_msg_offset = FORMAT1_HEADER_SIZE;
        for _n in 0..self.opc {
            let prop_msg = &(*msg)[prop_msg_offset..];
            let mut prop = Property::new();
            if !prop.parse(prop_msg) {
                return false;
            }
            prop_msg_offset += prop.size();
        }

        true
    }

}
