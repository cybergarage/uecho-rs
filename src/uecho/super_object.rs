// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub const ObjectOperatingStatus: u8 = 0x80;
pub const ObjectManufacturerCode: u8 = 0x8A;
pub const ObjectAnnoPropertyMap: u8 = 0x9D;
pub const ObjectSetPropertyMap: u8 = 0x9E;
pub const ObjectGetPropertyMap: u8 = 0x9F;

pub const ObjectOperatingStatusOn: u8 = 0x30;
pub const ObjectOperatingStatusOff: u8 = 0x31;
pub const ObjectOperatingStatusSize: i32 = 1;
pub const ObjectManufacturerEvaluationCodeMin: i32 = 0xFFFFF0;
pub const ObjectManufacturerEvaluationCodeMax: i32 = 0xFFFFFF;
pub const ObjectManufacturerCodeSize: i32 = 3;
pub const ObjectPropertyMapMaxSize: i32 = 16;
pub const ObjectAnnoPropertyMapMaxSize: i32 = (ObjectPropertyMapMaxSize + 1);
pub const ObjectSetPropertyMapMaxSize: i32 = (ObjectPropertyMapMaxSize + 1);
pub const ObjectGetPropertyMapMaxSize: i32 = (ObjectPropertyMapMaxSize + 1);

pub const ObjectManufacturerUnknown: i32 = ObjectManufacturerEvaluationCodeMin;
