// Copyright (C) 2022 Satoshi Konno All rights reserved.
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

/// Bytes offers encoding and decoding utility functions between byte array and integers.
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
