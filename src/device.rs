// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::object::Object;
use crate::property::*;
use crate::super_object::*;

pub const DEVICE_OPERATING_STATUS: u8 = OBJECT_OPERATING_STATUS;
pub const DEVICE_INSTALLATION_LOCATION: u8 = 0x81;
pub const DEVICE_STANDARD_VERSION: u8 = 0x82;
pub const DEVICE_IDENTIFICATION_NUMBER: u8 = 0x83;
pub const DEVICE_MEASURED_INSTANTANEOUS_POWER_CONSUMPTION: u8 = 0x84;
pub const DEVICE_MEASURED_CUMULATIVE_POWER_CONSUMPTION: u8 = 0x85;
pub const DEVICE_MANUFACTURER_FAULT_CODE: u8 = 0x86;
pub const DEVICE_CURRENT_LIMIT_SETTING: u8 = 0x87;
pub const DEVICE_FAULT_STATUS: u8 = 0x88;
pub const DEVICE_FAULT_DESCRIPTION: u8 = 0x89;
pub const DEVICE_MANUFACTURER_CODE: u8 = OBJECT_MANUFACTURER_CODE;
pub const DEVICE_BUSINESS_FACILITY_CODE: u8 = 0x8B;
pub const DEVICE_PRODUCT_CODE: u8 = 0x8C;
pub const DEVICE_PRODUCTION_NUMBER: u8 = 0x8D;
pub const DEVICE_PRODUCTION_DATE: u8 = 0x8E;
pub const DEVICE_POWER_SAVING_OPERATION_SETTING: u8 = 0x8F;
pub const DEVICE_REMOTE_CONTROL_SETTING: u8 = 0x93;
pub const DEVICE_CURRENT_TIME_SETTING: u8 = 0x97;
pub const DEVICE_CURRENT_DATE_SETTING: u8 = 0x98;
pub const DEVICE_POWER_LIMIT_SETTING: u8 = 0x99;
pub const DEVICE_CUMULATIVE_OPERATING_TIME: u8 = 0x9A;
pub const DEVICE_ANNO_PROPERTY_MAP: u8 = OBJECT_ANNO_PROPERTY_MAP;
pub const DEVICE_SET_PROPERTY_MAP: u8 = OBJECT_SET_PROPERTY_MAP;
pub const DEVICE_GET_PROPERTY_MAP: u8 = OBJECT_GET_PROPERTY_MAP;

pub const DEVICE_INSTALLATION_LOCATION_SIZE: usize = 1;
pub const DEVICE_STANDARD_VERSION_SIZE: usize = 4;
pub const DEVICE_IDENTIFICATION_NUMBER_SIZE: usize = 9;
pub const DEVICE_MEASURED_INSTANTANEOUS_POWER_CONSUMPTION_SIZE: usize = 2;
pub const DEVICE_MEASURED_CUMULATIVE_POWER_CONSUMPTION_SIZE: usize = 4;
pub const DEVICE_MANUFACTURER_FAULT_CODE_SIZE: usize = 255;
pub const DEVICE_CURRENT_LIMIT_SETTING_SIZE: usize = 1;
pub const DEVICE_FAULT_STATUS_SIZE: usize = 1;
pub const DEVICE_FAULT_DESCRIPTION_SIZE: usize = 2;
pub const DEVICE_MANUFACTURER_CODE_SIZE: usize = OBJECT_MANUFACTURER_CODE_SIZE;
pub const DEVICE_BUSINESS_FACILITY_CODE_SIZE: usize = 3;
pub const DEVICE_PRODUCT_CODE_SIZE: usize = 12;
pub const DEVICE_PRODUCTION_NUMBER_SIZE: usize = 12;
pub const DEVICE_PRODUCTION_DATE_SIZE: usize = 4;
pub const DEVICE_POWER_SAVING_OPERATION_SETTING_SIZE: usize = 1;
pub const DEVICE_REMOTE_CONTROL_SETTING_SIZE: usize = 1;
pub const DEVICE_CURRENT_TIME_SETTING_SIZE: usize = 2;
pub const DEVICE_CURRENT_DATE_SETTING_SIZE: usize = 4;
pub const DEVICE_POWER_LIMIT_SETTING_SIZE: usize = 2;
pub const DEVICE_CUMULATIVE_OPERATING_TIME_SIZE: usize = 5;
pub const DEVICE_ANNO_PROPERTY_MAP_SIZE: usize = OBJECT_ANNO_PROPERTY_MAP_MAX_SIZE;
pub const DEVICE_SET_PROPERTY_MAP_SIZE: usize = OBJECT_SET_PROPERTY_MAP_MAX_SIZE;
pub const DEVICE_GET_PROPERTY_MAP_SIZE: usize = OBJECT_GET_PROPERTY_MAP_MAX_SIZE;

pub const DEVICE_OPERATING_STATUS_ON: u8 = OBJECT_OPERATING_STATUS_ON;
pub const DEVICE_OPERATING_STATUS_OFF: u8 = OBJECT_OPERATING_STATUS_OFF;
pub const DEVICE_VERSION_APPENDIX_A: u8 = 'A' as u8;
pub const DEVICE_VERSION_APPENDIX_B: u8 = 'B' as u8;
pub const DEVICE_VERSION_APPENDIX_C: u8 = 'C' as u8;
pub const DEVICE_VERSION_APPENDIX_D: u8 = 'D' as u8;
pub const DEVICE_VERSION_APPENDIX_E: u8 = 'E' as u8;
pub const DEVICE_VERSION_APPENDIX_F: u8 = 'F' as u8;
pub const DEVICE_VERSION_APPENDIX_G: u8 = 'G' as u8;
pub const DEVICE_VERSION_UNKNOWN: u8 = 0;
pub const DEVICE_DEFAULT_VERSION_APPENDIX: u8 = DEVICE_VERSION_APPENDIX_G;
pub const DEVICE_FAULT_OCCURRED: u8 = 0x41;
pub const DEVICE_NO_FAULT_OCCURRED: u8 = 0x42;
pub const DEVICE_INSTALLATION_LOCATION_UNKNOWN: u8 = 0x00;
pub const DEVICE_MANUFACTURER_UNKNOWN: u32 = OBJECT_MANUFACTURER_UNKNOWN;

pub struct Device {}

impl Device {
    pub fn new() -> Object {
        let mut obj = Object::new();
        Device::add_mandatory_properties(&mut obj);
        obj
    }

    fn add_mandatory_properties(obj: &mut Object) {
        obj.set_property(OBJECT_OPERATING_STATUS, PROPERTY_ATTRIBUTE_READ_ANNO);
        obj.set_operating_status(true);

        obj.set_property(DEVICE_INSTALLATION_LOCATION, PROPERTY_ATTRIBUTE_READ_ANNO);
        obj.set_installation_location(DEVICE_INSTALLATION_LOCATION_UNKNOWN);

        obj.set_property(DEVICE_STANDARD_VERSION, PROPERTY_ATTRIBUTE_READ);
        obj.set_standard_version(DEVICE_STANDARD_VERSION);

        obj.set_property(DEVICE_FAULT_STATUS, PROPERTY_ATTRIBUTE_READ_ANNO);
        obj.set_fault_status(false);

        obj.set_property(DEVICE_MANUFACTURER_CODE, PROPERTY_ATTRIBUTE_READ);
    }
}

impl Object {
    pub fn set_operating_status(&mut self, status: bool) -> bool {
        let status_byte: u8 = if status {
            OBJECT_OPERATING_STATUS_ON
        } else {
            OBJECT_OPERATING_STATUS_OFF
        };
        self.set_property_byte(DEVICE_OPERATING_STATUS, status_byte)
    }

    pub fn operating_status(&mut self) -> &mut Property {
        self.property(DEVICE_OPERATING_STATUS).unwrap()
    }

    pub fn set_installation_location(&mut self, loc: u8) -> bool {
        self.set_property_byte(DEVICE_INSTALLATION_LOCATION, loc)
    }

    pub fn installation_location(&mut self) -> &mut Property {
        self.property(DEVICE_INSTALLATION_LOCATION).unwrap()
    }

    pub fn set_standard_version(&mut self, ver: u8) -> bool {
        let mut vers: [u8; 4] = [0; 4];
        vers[2] = ver;
        self.set_property_bytes(DEVICE_STANDARD_VERSION, &vers)
    }

    pub fn standard_version(&mut self) -> &mut Property {
        self.property(DEVICE_STANDARD_VERSION).unwrap()
    }

    pub fn set_fault_status(&mut self, status: bool) -> bool {
        let status_byte: u8 = if status {
            DEVICE_FAULT_OCCURRED
        } else {
            DEVICE_NO_FAULT_OCCURRED
        };
        self.set_property_byte(DEVICE_FAULT_STATUS, status_byte)
    }

    pub fn fault_status(&mut self) -> &mut Property {
        self.property(DEVICE_FAULT_STATUS).unwrap()
    }
}