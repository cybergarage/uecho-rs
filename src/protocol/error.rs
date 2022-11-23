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

use hex;
use std::fmt;

#[derive(Debug, Clone)]
pub struct MessageError {
    pub message: String,
    pub offset: usize,
}

impl MessageError {
    pub fn new(msg_bytes: &[u8], offset: usize) -> MessageError {
        MessageError {
            message: hex::encode(msg_bytes),
            offset: offset,
        }
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (offset:{})", self.message, self.offset)
    }
}

impl std::error::Error for MessageError {}
