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
#[cfg(feature = "std")]
use uuid::Uuid;

/// OID generates a unique identification number wit the specified manufacture code.
pub struct OID {
    data: Vec<u8>,
}

impl OID {
    pub fn new(code: ObjectCode) -> OID {
        // 6.11.1 Node Profile Class: Detailed Specifications
        let mut oid = vec![0 as u8; 17];
        oid[0] = 0xFE;
        let mut man = vec![0 as u8; 3];
        Bytes::from_u32(code, &mut man);
        oid[1] = man[0];
        oid[2] = man[1];
        oid[3] = man[2];
        #[cfg(not(feature = "std"))]
        {
            oid[16] = 0x01;
        }
        #[cfg(feature = "std")]
        {
            let v4_bytes = Uuid::new_v4().into_bytes();
            for n in 0..=12 {
                oid[4 + n] = v4_bytes[v4_bytes.len() - n - 1];
            }
        }
        OID { data: oid }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
}
