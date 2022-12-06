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
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::protocol::property::*;
use crate::protocol::Esv;

pub const HEADER_EHD1_ECHONET: u8 = 0x10;
pub const HEADER_EHD2_FORMAT1: u8 = 0x81;
pub const FRAME_HEADER_SIZE: usize = 1 + 1 + 2;
pub const FORMAT1_HEADER_SIZE: usize = 3 + 3 + 1 + 1;

/// TID represents an ECHONET-Lite transaction identification number as specified in the ECHONET-Lite specification.
pub type TID = u16;
pub const TID_MIN: TID = 0;
pub const TID_MAX: TID = 65535;

/// Message represents a messaging packet between ECHONET-Lite nodes as specified in the ECHONET-Lite specification.
/// # Examples
/// ```
/// use echonet::protocol::Message;
/// use echonet::protocol::Esv;
/// use echonet::protocol::Property;
/// use hex;
///
/// let mut msg = Message::new();
/// msg.set_esv(Esv::ReadRequest);
/// msg.set_deoj(0x029101);
/// let mut prop = Property::new();
/// prop.set_code(0x80);
/// msg.add_property(prop);
///
/// let msg = Message::from_bytes(&msg.bytes());
/// let opc = msg.opc();
/// for n in 0..opc {
///     let prop = msg.property(n);
///     println!("[{}] {}", n, hex::encode(prop.data()));
/// }
/// for (n, prop) in msg.properties().iter().enumerate() {
///     println!("[{}] {}", n, hex::encode(prop.data()));
/// }
/// ```
pub struct Message {
    ehd: [u8; 2],
    tid: [u8; 2],
    seoj: [u8; 3],
    deoj: [u8; 3],
    esv: u8,
    properties: Vec<Property>,
    set_properties: Vec<Property>,
    get_properties: Vec<Property>,
    from: SocketAddr,
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
            set_properties: Vec::<Property>::new(),
            get_properties: Vec::<Property>::new(),
            from: SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
        }
    }

    pub fn from_bytes(msg_bytes: &[u8]) -> Message {
        let mut msg = Message::new();
        msg.parse(msg_bytes);
        msg
    }

    pub fn set_tid(&mut self, tid: TID) -> &mut Self {
        self.tid[0] = ((tid & 0xFF00) >> 8) as u8;
        self.tid[1] = (tid & 0x00FF) as u8;
        self
    }

    pub fn has_tid(&self) -> bool {
        if self.tid() == 0 {
            return false;
        }
        true
    }

    pub fn tid(&self) -> TID {
        ((self.tid[0] as TID) << 8) + (self.tid[1] as TID)
    }

    fn to_object_code(&self, eoj: &[u8]) -> u32 {
        ((eoj[0] as u32) << 16) + ((eoj[1] as u32) << 8) + (eoj[2] as u32)
    }

    pub fn set_seoj(&mut self, code: u32) -> &mut Self {
        self.seoj[0] = ((code & 0xFF0000) >> 16) as u8;
        self.seoj[1] = ((code & 0xFF00) >> 8) as u8;
        self.seoj[2] = (code & 0x00FF) as u8;
        self
    }

    pub fn seoj(&self) -> u32 {
        self.to_object_code(&self.seoj)
    }

    pub fn set_deoj(&mut self, code: u32) -> &mut Self {
        self.deoj[0] = ((code & 0xFF0000) >> 16) as u8;
        self.deoj[1] = ((code & 0xFF00) >> 8) as u8;
        self.deoj[2] = (code & 0x00FF) as u8;
        self
    }

    pub fn deoj(&self) -> u32 {
        self.to_object_code(&self.deoj)
    }

    pub fn set_esv(&mut self, code: Esv) -> &mut Self {
        self.esv = code as u8;
        self
    }

    pub fn esv(&self) -> Esv {
        Esv::from_u8(self.esv)
    }

    pub fn opc(&self) -> usize {
        self.properties.len()
    }

    pub fn opc_set(&self) -> usize {
        self.set_properties.len()
    }

    pub fn opc_get(&self) -> usize {
        self.get_properties.len()
    }

    pub fn add_property(&mut self, prop: Property) -> &mut Self {
        self.properties.push(prop);
        self
    }

    pub fn properties(&self) -> &Vec<Property> {
        &self.properties
    }

    pub fn set_properties(&self) -> &Vec<Property> {
        &self.set_properties
    }

    pub fn get_properties(&self) -> &Vec<Property> {
        &self.get_properties
    }

    pub fn property(&self, n: usize) -> &Property {
        &self.properties[n]
    }

    pub fn set_from(&mut self, addr: SocketAddr) -> &mut Self {
        self.from = addr;
        self
    }

    pub fn from(&self) -> SocketAddr {
        self.from
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

        fn parse_property_bytes(
            props: &mut Vec<Property>,
            msg: &[u8],
            prop_msg_offset: &mut usize,
        ) -> bool {
            let opc = msg[*prop_msg_offset] as usize;
            *prop_msg_offset += 1 as usize;
            for _n in 0..opc {
                let prop_msg = &(*msg)[*prop_msg_offset..];
                let mut prop = Property::new();
                if !prop.parse(prop_msg) {
                    return false;
                }
                *prop_msg_offset += FORMAT1_PROPERTY_HEADER_SIZE + prop.size();
                props.push(prop);
            }

            true
        }

        let prop_msg = &(*msg)[7..];
        let mut prop_msg_offset = 0 as usize;
        match self.esv() {
            Esv::WriteReadRequest | Esv::WriteReadResponse => {
                let properties = &mut self.set_properties;
                if !parse_property_bytes(properties, prop_msg, &mut prop_msg_offset) {
                    return false;
                }
                let properties = &mut self.get_properties;
                if !parse_property_bytes(properties, prop_msg, &mut prop_msg_offset) {
                    return false;
                }
            }
            _ => {
                let properties = &mut self.properties;
                if !parse_property_bytes(properties, prop_msg, &mut prop_msg_offset) {
                    return false;
                }
            }
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

    fn add_property_bytes(&self, msg_bytes: &mut Vec<u8>, props: &Vec<Property>) {
        msg_bytes.push(props.len() as u8);
        for prop in props {
            msg_bytes.push(prop.code());
            let pdc = prop.size();
            msg_bytes.push(pdc as u8);
            let prop_data = prop.data();
            for j in 0..pdc {
                msg_bytes.push(prop_data[j]);
            }
        }
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

        match self.esv() {
            Esv::WriteReadRequest | Esv::WriteReadResponse => {
                self.add_property_bytes(&mut msg_bytes, &self.set_properties);
                self.add_property_bytes(&mut msg_bytes, &self.get_properties);
            }
            _ => {
                self.add_property_bytes(&mut msg_bytes, &self.properties);
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
            let res = f.write_fmt(format_args!("{:02X}", b));
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }
}
