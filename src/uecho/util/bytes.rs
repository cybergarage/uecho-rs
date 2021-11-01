// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub struct Bytes {}

impl Bytes {
    pub fn from_u32(val: u32, bytes: &mut [u8]) {
        let bytes_size = bytes.len();
        for n in 0..bytes_size {
            let idx = (bytes_size - 1) - n;
            bytes[idx] = ((val >> (n * 8)) & 0xFF) as u8;
        }
    }
    pub fn to_u32(bytes: &[u8]) -> u32 {
        let mut val = 0 as u32;
        let bytes_size = bytes.len();
        for n in 0..bytes_size {
            let idx = (bytes_size - 1) - n;
            val += (bytes[idx] as u32) << (n * 8);
        }
        val
    }
}
