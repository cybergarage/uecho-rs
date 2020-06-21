// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub const FORMAT1_PROPERTY_HEADER_SIZE: usize = 2;

pub struct Property {
    code: u8,
    data: Vec<u8>,
}

impl Property {
    pub fn new() -> Property {
        Property {
            code: 0,
            data: Vec::new(),
        }
    }

    pub fn code(&self) -> u8 {
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
        if msg_len <= FORMAT1_PROPERTY_HEADER_SIZE {
            return false;
        }

        self.code = msg[0];
        let data_size = msg[1] as usize;

        if msg_len < (FORMAT1_PROPERTY_HEADER_SIZE + data_size) {
            return false;
        }

        self.data.clear();

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
