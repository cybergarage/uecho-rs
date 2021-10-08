// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::object::Object;
use crate::uecho::property::*;
use crate::uecho::super_object::*;

pub const DeviceOperatingStatus: u8 = ObjectOperatingStatus;
pub const DeviceInstallationLocation: u8 = 0x81;
pub const DeviceStandardVersion: u8 = 0x82;
pub const DeviceIdentificationNumber: u8 = 0x83;
pub const DeviceMeasuredInstantaneousPowerConsumption: u8 = 0x84;
pub const DeviceMeasuredCumulativePowerConsumption: u8 = 0x85;
pub const DeviceManufacturerFaultCode: u8 = 0x86;
pub const DeviceCurrentLimitSetting: u8 = 0x87;
pub const DeviceFaultStatus: u8 = 0x88;
pub const DeviceFaultDescription: u8 = 0x89;
pub const DeviceManufacturerCode: u8 = ObjectManufacturerCode;
pub const DeviceBusinessFacilityCode: u8 = 0x8B;
pub const DeviceProductCode: u8 = 0x8C;
pub const DeviceProductionNumber: u8 = 0x8D;
pub const DeviceProductionDate: u8 = 0x8E;
pub const DevicePowerSavingOperationSetting: u8 = 0x8F;
pub const DeviceRemoteControlSetting: u8 = 0x93;
pub const DeviceCurrentTimeSetting: u8 = 0x97;
pub const DeviceCurrentDateSetting: u8 = 0x98;
pub const DevicePowerLimitSetting: u8 = 0x99;
pub const DeviceCumulativeOperatingTime: u8 = 0x9A;
pub const DeviceAnnoPropertyMap: u8 = ObjectAnnoPropertyMap;
pub const DeviceSetPropertyMap: u8 = ObjectSetPropertyMap;
pub const DeviceGetPropertyMap: u8 = ObjectGetPropertyMap;

pub const DeviceInstallationLocationSize: i32 = 1;
pub const DeviceStandardVersionSize: i32 = 4;
pub const DeviceIdentificationNumberSize: i32 = 9;
pub const DeviceMeasuredInstantaneousPowerConsumptionSize: i32 = 2;
pub const DeviceMeasuredCumulativePowerConsumptionSize: i32 = 4;
pub const DeviceManufacturerFaultCodeSize: i32 = 255;
pub const DeviceCurrentLimitSettingSize: i32 = 1;
pub const DeviceFaultStatusSize: i32 = 1;
pub const DeviceFaultDescriptionSize: i32 = 2;
pub const DeviceManufacturerCodeSize: i32 = ObjectManufacturerCodeSize;
pub const DeviceBusinessFacilityCodeSize: i32 = 3;
pub const DeviceProductCodeSize: i32 = 12;
pub const DeviceProductionNumberSize: i32 = 12;
pub const DeviceProductionDateSize: i32 = 4;
pub const DevicePowerSavingOperationSettingSize: i32 = 1;
pub const DeviceRemoteControlSettingSize: i32 = 1;
pub const DeviceCurrentTimeSettingSize: i32 = 2;
pub const DeviceCurrentDateSettingSize: i32 = 4;
pub const DevicePowerLimitSettingSize: i32 = 2;
pub const DeviceCumulativeOperatingTimeSize: i32 = 5;
pub const DeviceAnnoPropertyMapSize: i32 = ObjectAnnoPropertyMapMaxSize;
pub const DeviceSetPropertyMapSize: i32 = ObjectSetPropertyMapMaxSize;
pub const DeviceGetPropertyMapSize: i32 = ObjectGetPropertyMapMaxSize;

pub const DeviceOperatingStatusOn: u8 = ObjectOperatingStatusOn;
pub const DeviceOperatingStatusOff: u8 = ObjectOperatingStatusOff;
pub const DeviceVersionAppendixA: u8 = 'A' as u8;
pub const DeviceVersionAppendixB: u8 = 'B' as u8;
pub const DeviceVersionAppendixC: u8 = 'C' as u8;
pub const DeviceVersionAppendixD: u8 = 'D' as u8;
pub const DeviceVersionAppendixE: u8 = 'E' as u8;
pub const DeviceVersionAppendixF: u8 = 'F' as u8;
pub const DeviceVersionAppendixG: u8 = 'G' as u8;
pub const DeviceVersionUnknown: u8 = 0;
pub const DeviceDefaultVersionAppendix: u8 = DeviceVersionAppendixG;
pub const DeviceFaultOccurred: u8 = 0x41;
pub const DeviceNoFaultOccurred: u8 = 0x42;
pub const DeviceInstallationLocationUnknown: u8 = 0x00;
pub const DeviceManufacturerUnknown: i32 = ObjectManufacturerUnknown;

pub struct Device {}

impl Device {
    pub fn new() -> Object {
        let mut obj = Object::new();
        add_mandatory_properties(&mut obj);
        obj
    }
}

impl Object {
    pub fn set_operating_status(&mut self, status: bool) -> bool {
        let status_byte: u8 = if status {
            ObjectOperatingStatusOn
        } else {
            ObjectOperatingStatusOff
        };
        self.set_property_byte(DeviceOperatingStatus, status_byte)
    }

    pub fn operating_status(&mut self) -> &mut Property {
        self.property(DeviceOperatingStatus).unwrap()
    }
}

fn add_mandatory_properties(obj: &mut Object) {
    obj.set_property(ObjectOperatingStatus, PropertyAttributeReadAnno);
    obj.set_operating_status(true);

    obj.set_property(DeviceInstallationLocation, PropertyAttributeReadAnno);
    obj.set_property(DeviceStandardVersion, PropertyAttributeRead);
    obj.set_property(DeviceFaultStatus, PropertyAttributeReadAnno);
    obj.set_property(DeviceManufacturerCode, PropertyAttributeRead);
}
