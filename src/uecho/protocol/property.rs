// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub const FORMAT1_PROPERTY_HEADER_SIZE: usize = 2;

pub struct Property {
    code: u8,
    data: Vec::<u8>,
    //data: [u8]
    //data: &'a [u8],
}

impl Property {
    pub fn new() -> Property {
        Property { 
            code: 0,
            data: Vec::<u8>::new(), }
    }

    pub fn code(&self) -> u8 {
        return self.code
    }

    pub fn size(&self) -> usize {
        return self.data.len();
    }

    pub fn parse(&mut self, msg: &[u8]) -> bool {
        let msg_len = msg.len();
        if msg_len <= FORMAT1_PROPERTY_HEADER_SIZE {
            return false;
        }

        self.code = msg[0];
        let data_size = msg[1] as usize;

        if msg_len <= (FORMAT1_PROPERTY_HEADER_SIZE + data_size) {
            return false;
        }

        self.data.clear();
        self.data.clone_from_slice(&msg[2..]);

        true
    }
}
