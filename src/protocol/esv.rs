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

use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// ESV represents an ECHONET-Lite service (ESV) code as specified in the ECHONET-Lite specification.
#[derive(Copy, Clone, PartialEq, Debug, EnumIter)]
pub enum ESV {
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

impl ESV {
    pub fn from_u8(x: u8) -> ESV {
        for esv in ESV::iter() {
            if x == (esv as u8) {
                return esv;
            }
        }
        ESV::Unknown
    }

    pub fn to_u8(esv: ESV) -> u8 {
        esv as u8
    }

    pub fn is_request(&self) -> bool {
        match self {
            ESV::WriteRequest => return true,
            ESV::WriteRequestResponseRequired => return true,
            ESV::ReadRequest => return true,
            ESV::NotificationRequest => return true,
            ESV::WriteReadRequest => return true,
            ESV::NotificationResponseRequired => return true,
            _ => return false,
        }
    }

    pub fn is_response(&self) -> bool {
        match self {
            ESV::WriteResponse => return true,
            ESV::ReadResponse => return true,
            ESV::Notification => return true,
            ESV::NotificationResponse => return true,
            ESV::WriteReadResponse => return true,
            ESV::WriteRequestError => return true,
            ESV::WriteRequestResponseRequiredError => return true,
            ESV::ReadRequestError => return true,
            ESV::NotificationRequestError => return true,
            ESV::WriteReadRequestError => return true,
            _ => return false,
        }
    }

    pub fn is_error_response(&self) -> bool {
        match self {
            ESV::WriteRequestError => return true,
            ESV::WriteRequestResponseRequiredError => return true,
            ESV::ReadRequestError => return true,
            ESV::NotificationRequestError => return true,
            ESV::WriteReadRequestError => return true,
            _ => return false,
        }
    }

    pub fn is_notification_response(&self) -> bool {
        match self {
            ESV::Notification => return true,
            ESV::NotificationResponse => return true,
            _ => return false,
        }
    }

    pub fn is_unicast_response(&self) -> bool {
        if !self.is_response() {
            return false;
        }
        !self.is_notification_response()
    }
}

impl fmt::Display for ESV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = *self as u8;
        let res = f.write_fmt(format_args!("{:02X}", code));
        if res.is_err() {
            return res;
        }
        Ok(())
    }
}
