// Copyright (C) 2022 The uecho-rs Authors All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::net::{IpAddr, Ipv4Addr};

use crate::protocol::esv::*;
use crate::protocol::property::*;

pub const HEADER_EHD1_ECHONET: u8 = 0x10;
pub const HEADER_EHD2_FORMAT1: u8 = 0x81;
pub const FRAME_HEADER_SIZE: usize = 1 + 1 + 2;
pub const FORMAT1_HEADER_SIZE: usize = 3 + 3 + 1 + 1;

pub type TID = u16;
pub const TID_MIN: TID = 0;
pub const TID_MAX: TID = 65535;

pub struct Message {
    ehd: [u8; 2],
    tid: [u8; 2],
    seoj: [u8; 3],
    deoj: [u8; 3],
    esv: u8,
    properties: Vec<Property>,
    addr: IpAddr,
}

impl Message {
    pub fn new() -> Message {
        Message {
            ehd: [HEADER_EHD1_ECHONET, HEADER_EHD2_FORMAT1],
            tid: [0; 2],
            seoj: [0; 3],
            deoj: [0; 3],
            esv: 0,
            properties: Vec::<Property>::new(),
            addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        }
    }

    pub fn set_tid(&mut self, tid: TID) {
        self.tid[0] = ((tid & 0xFF00) >> 8) as u8;
        self.tid[1] = (tid & 0x00FF) as u8;
    }

    pub fn tid(&self) -> TID {
        ((self.tid[0] as TID) << 8) + (self.tid[1] as TID)
    }

    fn to_object_code(&self, eoj: &[u8]) -> u32 {
        ((eoj[0] as u32) << 16) + ((eoj[1] as u32) << 8) + (eoj[2] as u32)
    }

    pub fn set_seoj(&mut self, code: u32) {
        self.seoj[0] = ((code & 0xFF0000) >> 16) as u8;
        self.seoj[1] = ((code & 0xFF00) >> 8) as u8;
        self.seoj[2] = (code & 0x00FF) as u8;
    }

    pub fn seoj(&self) -> u32 {
        self.to_object_code(&self.seoj)
    }

    pub fn set_deoj(&mut self, code: u32) {
        self.deoj[0] = ((code & 0xFF0000) >> 16) as u8;
        self.deoj[1] = ((code & 0xFF00) >> 8) as u8;
        self.deoj[2] = (code & 0x00FF) as u8;
    }

    pub fn deoj(&self) -> u32 {
        self.to_object_code(&self.deoj)
    }

    pub fn set_esv(&mut self, code: Esv) {
        self.esv = code as u8
    }

    pub fn esv(&self) -> Esv {
        Esv::from_u8(self.esv)
    }

    pub fn opc(&self) -> usize {
        self.properties.len()
    }

    pub fn add_property(&mut self, prop: Property) {
        self.properties.push(prop);
    }

    pub fn properties(&self) -> &Vec<Property> {
        &self.properties
    }

    pub fn property(&self, n: usize) -> &Property {
        &self.properties[n]
    }

    pub fn set_addr(&mut self, addr: IpAddr) {
        self.addr = addr
    }

    pub fn addr(&self) -> IpAddr {
        self.addr
    }

    pub fn is_format1(&mut self) -> bool {
        if (self.ehd[0] != HEADER_EHD1_ECHONET) || (self.ehd[1] != HEADER_EHD2_FORMAT1) {
            return false;
        }

        true
    }

    pub fn parse(&mut self, msg: &[u8]) -> bool {
        if !self.parse_header(msg) {
            return false;
        }

        if !self.is_format1() {
            return false;
        }

        let format1_msg = &(*msg)[4..];
        if !self.parse_format1_data(format1_msg) {
            return false;
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
        if msg.len() < FORMAT1_HEADER_SIZE {
            return false;
        }

        self.seoj = [msg[0], msg[1], msg[2]];
        self.deoj = [msg[3], msg[4], msg[5]];
        self.esv = msg[6];

        let opc = msg[7] as usize;
        let mut prop_msg_offset = FORMAT1_HEADER_SIZE;
        for _n in 0..opc {
            let prop_msg = &(*msg)[prop_msg_offset..];
            let mut prop = Property::new();
            if !prop.parse(prop_msg) {
                return false;
            }
            prop_msg_offset += FORMAT1_PROPERTY_HEADER_SIZE + prop.size();
            self.properties.push(prop);
        }

        true
    }

    pub fn equals(&self, msg: &Message) -> bool {
        if self.tid() != msg.tid() {
            return false;
        }
        if self.seoj() != msg.seoj() {
            return false;
        }
        if self.deoj() != msg.deoj() {
            return false;
        }
        if self.esv() != msg.esv() {
            return false;
        }
        if self.opc() != msg.opc() {
            return false;
        }

        let opc = msg.opc();
        for n in 0..opc {
            let prop = self.property(n);
            if !prop.equals(msg.property(n)) {
                return false;
            }
        }

        true
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut msg_bytes: Vec<u8> = Vec::new();

        msg_bytes.push(self.ehd[0]);
        msg_bytes.push(self.ehd[1]);
        msg_bytes.push(self.tid[0]);
        msg_bytes.push(self.tid[1]);
        msg_bytes.push(self.seoj[0]);
        msg_bytes.push(self.seoj[1]);
        msg_bytes.push(self.seoj[2]);
        msg_bytes.push(self.deoj[0]);
        msg_bytes.push(self.deoj[1]);
        msg_bytes.push(self.deoj[2]);
        msg_bytes.push(self.esv);

        let opc = self.opc();
        msg_bytes.push(opc as u8);
        for i in 0..opc {
            let prop = self.property(i);
            msg_bytes.push(prop.code());
            let pdc = prop.size();
            msg_bytes.push(pdc as u8);
            let prop_data = prop.data();
            for j in 0..pdc {
                msg_bytes.push(prop_data[j]);
            }
        }
        msg_bytes
    }
}

impl Clone for Message {
    fn clone(&self) -> Message {
        let mut msg = Message::new();
        msg.parse(&self.bytes());
        msg
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in self.bytes() {
            let res = f.write_fmt(format_args!("{:X}", b));
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }
}
