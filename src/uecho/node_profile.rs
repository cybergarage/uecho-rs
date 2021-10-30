// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::super_object::*;

pub const NodeProfileObject: u32 = 0x0EF001;
pub const NodeProfileObjectReadOnly: u32 = 0x0EF002;
pub const NodeProfileClassCode: u8 = 0xF0;
pub const NodeProfileInstanceGeneralCode: u8 = 0x01;
pub const NodeProfileInstanceTransmissionOnlyCode: u8 = 0x02;

pub const NodeProfileClassOperatingStatus: u8 = ObjectOperatingStatus;
pub const NodeProfileClassVersionInformation: u8 = 0x82;
pub const NodeProfileClassIdentificationNumber: u8 = 0x83;
pub const NodeProfileClassFaultContent: u8 = 0x89;
pub const NodeProfileClassUniqueIdentifierData: u8 = 0xBF;
pub const NodeProfileClassNumberOfSelfNodeInstances: u8 = 0xD3;
pub const NodeProfileClassNumberOfSelfNodeClasses: u8 = 0xD4;
pub const NodeProfileClassInstanceListNotification: u8 = 0xD5;
pub const NodeProfileClassSelfNodeInstanceListS: u8 = 0xD6;
pub const NodeProfileClassSelfNodeClassListS: u8 = 0xD7;

pub const NodeProfileClassOperatingStatusSize: i32 = 1;
pub const NodeProfileClassVersionInformationSize: i32 = 4;
pub const NodeProfileClassIdentificationManufacturerCodeSize: i32 = 3;
pub const NodeProfileClassIdentificationUniqueIdSize: i32 = 13;
pub const NodeProfileClassIdentificationNumberSize: i32 = 1
    + NodeProfileClassIdentificationManufacturerCodeSize
    + NodeProfileClassIdentificationUniqueIdSize;
pub const NodeProfileClassFaultContentSize: i32 = 2;
pub const NodeProfileClassUniqueIdentifierDataSize: i32 = 2;
pub const NodeProfileClassNumberOfSelfNodeInstancesSize: i32 = 3;
pub const NodeProfileClassNumberOfSelfNodeClassesSize: i32 = 2;
pub const NodeProfileClassSelfNodeInstanceListSMax: i32 = 0xFF;
pub const NodeProfileClassSelfNodeClassListSMax: i32 = 0xFF;
pub const NodeProfileClassInstanceListNotificationMax: i32 =
    NodeProfileClassSelfNodeInstanceListSMax;

pub const NodeProfileClassOperatingStatusOn: u8 = ObjectOperatingStatusOn;
pub const NodeProfileClassOperatingStatusOff: u8 = ObjectOperatingStatusOff;
pub const NodeProfileClassBooting: u8 = 0x30;
pub const NodeProfileClassNotBooting: u8 = 0x31;
pub const LowerCommunicationLayerProtocolType: u8 = 0xFE;
