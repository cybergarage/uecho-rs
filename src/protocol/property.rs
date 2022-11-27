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

pub const FORMAT1_PROPERTY_HEADER_SIZE: usize = 2;

/// PropertyCode represents an ECHONET-Lite property code (EPC) in a ECHONET-Lite message.
pub type PropertyCode = u8;

/// Each ECHONET-Lite message has properties. Property represents a ECHONET-Lite property of a message packet as specified in the ECHONET-Lite specification.
pub struct Property {
    code: PropertyCode,
    data: Vec<u8>,
}

impl Property {
    pub fn new() -> Property {
        Property {
            code: 0,
            data: Vec::new(),
        }
    }

    pub fn set_code(&mut self, code: PropertyCode) {
        self.code = code
    }

    pub fn code(&self) -> PropertyCode {
        self.code
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn parse(&mut self, msg: &[u8]) -> bool {
        let msg_len = msg.len();
        if msg_len < FORMAT1_PROPERTY_HEADER_SIZE {
            return false;
        }

        self.code = msg[0];
        let data_size = msg[1] as usize;

        self.data.clear();
        if data_size == 0 {
            return true;
        }

        if msg_len < (FORMAT1_PROPERTY_HEADER_SIZE + data_size) {
            return false;
        }

        let prop_data = &(*msg)[2..];
        for n in 0..data_size {
            self.data.push(prop_data[n]);
        }

        true
    }

    pub fn equals(&self, prop: &Property) -> bool {
        if self.code() != prop.code() {
            return false;
        }
        if self.size() != prop.size() {
            return false;
        }

        for n in 0..self.size() {
            if self.data[n] != prop.data[n] {
                return false;
            }
        }

        true
    }
}
