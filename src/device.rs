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

use std::sync::Arc;
use std::sync::Mutex;

use crate::device_node::DeviceNode;
use crate::local_node::LocalNode;
use crate::object::{Object, ObjectCode};
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

pub struct Device {
    node: Arc<Mutex<DeviceNode>>,
}

impl Device {
    pub fn new(code: ObjectCode) -> Device {
        Device {
            node: DeviceNode::new(),
        }
    }

    pub fn new_with_node(code: ObjectCode, node: Arc<Mutex<LocalNode>>) -> Device {
        Device {
            node: DeviceNode::new_with_node(node),
        }
    }

    pub fn start(&mut self) -> bool {
        let mut dev = self.node.lock().unwrap();
        dev.start();
        dev.add_observer(Arc::new(Mutex::new(self.node.clone())));

        let local_node = dev.local_node();
        local_node
            .lock()
            .unwrap()
            .add_observer(Arc::new(Mutex::new(local_node.clone())));

        true
    }

    pub fn stop(&mut self) -> bool {
        let mut ctrl = self.node.lock().unwrap();
        ctrl.stop()
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
        self.find_property_mut(DEVICE_OPERATING_STATUS).unwrap()
    }

    pub fn set_installation_location(&mut self, loc: u8) -> bool {
        self.set_property_byte(DEVICE_INSTALLATION_LOCATION, loc)
    }

    pub fn installation_location(&mut self) -> &mut Property {
        self.find_property_mut(DEVICE_INSTALLATION_LOCATION)
            .unwrap()
    }

    pub fn set_standard_version(&mut self, ver: u8) -> bool {
        let mut vers: [u8; 4] = [0; 4];
        vers[2] = ver;
        self.set_property_bytes(DEVICE_STANDARD_VERSION, &vers)
    }

    pub fn standard_version(&mut self) -> &mut Property {
        self.find_property_mut(DEVICE_STANDARD_VERSION).unwrap()
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
        self.find_property_mut(DEVICE_FAULT_STATUS).unwrap()
    }
}
