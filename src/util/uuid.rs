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

use crate::object::ObjectCode;
use crate::util::Bytes;
use uuid::Uuid;

/// UUID generates a unique identification number wit the specified manufacture code.
pub struct UUID {
    data: Vec<u8>,
}

impl UUID {
    pub fn new(code: ObjectCode) -> UUID {
        // 6.11.1 Node Profile Class: Detailed Specifications
        let mut uuid = vec![0 as u8; 17];
        uuid[0] = 0xFE;
        let mut man = vec![0 as u8; 3];
        Bytes::from_u32(code, &mut man);
        uuid[1] = man[0];
        uuid[2] = man[1];
        uuid[3] = man[2];
        let v4_bytes = Uuid::new_v4().into_bytes();
        for n in 0..=12 {
            uuid[4 + n] = v4_bytes[v4_bytes.len() - n - 1];
        }
        UUID { data: uuid }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
}
