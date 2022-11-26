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

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// ESV represents a ECHONET-Lite service (ESV) code as specified in the ECHONET-lite specification.
#[derive(Copy, Clone, PartialEq, Debug, EnumIter)]
pub enum Esv {
    Unknown = 0x00,
    WriteRequest = 0x60,
    WriteRequestResponseRequired = 0x61,
    ReadRequest = 0x62,
    NotificationRequest = 0x63,
    WriteReadRequest = 0x6E,
    WriteResponse = 0x71,
    ReadResponse = 0x72,
    Notification = 0x73,
    NotificationResponseRequired = 0x74,
    NotificationResponse = 0x7A,
    WriteReadResponse = 0x7E,
    WriteRequestError = 0x50,
    WriteRequestResponseRequiredError = 0x51,
    ReadRequestError = 0x52,
    NotificationRequestError = 0x53,
    WriteReadRequestError = 0x5E,
}

impl Esv {
    pub fn from_u8(x: u8) -> Esv {
        for esv in Esv::iter() {
            if x == (esv as u8) {
                return esv;
            }
        }
        Esv::Unknown
    }

    pub fn to_u8(esv: Esv) -> u8 {
        esv as u8
    }
}
