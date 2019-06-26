// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub const FORMAT1_PROPERTY_HEADER_SIZE: usize = 2;

pub struct Property {
    code: u8,
    //data: [u8]
    //data: &'a [u8],
}

impl Property {
    pub fn new() -> Property {
        Property { code: 0 }
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn size(&self) -> usize {
        //self.data.size()
        0
    }

    pub fn parse(&mut self, msg: &[u8]) -> bool {
        if msg.len() <= FORMAT1_PROPERTY_HEADER_SIZE {
            return false;
        }

        self.code = msg[0];
        let _data_size = msg[1] as usize;

        //let _data = &msg[2..];
        //self.data = &msg[2..];

        true
    }
}
