// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
