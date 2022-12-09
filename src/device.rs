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

#![allow(dead_code)]

use std::sync::Arc;
use std::sync::Mutex;

use crate::device_node::DeviceNode;
use crate::node::Node;
use crate::object::{Object, ObjectCode};
use crate::property::PropertyCode;
use crate::request_handler::*;
use crate::super_object::*;
use crate::util::Bytes;

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
pub const DEVICE_MANUFACTURER_EXPERIMENT: u32 = OBJECT_MANUFACTURER_EXPERIMENT;

/// Device represents an ECHONET-Lite device node.
/// # Examples
/// ```
/// use std::sync::{Arc, Mutex};
/// use echonet::protocol::{Esv, Property};
/// use echonet::util::Bytes;
/// use echonet::{Device, Object, ObjectCode, RequestHandler};
///
/// pub struct MyDevice {
///     device: Device,
///     on: bool,
/// }
///
/// impl MyDevice {
///     pub fn new() -> Arc<Mutex<MyDevice>> {
///         let m = Arc::new(Mutex::new(MyDevice {
///             device: Device::new(0x029101),
///             on: false,
///         }));
///         m.lock().unwrap().device.set_request_handler(m.clone());
///         m
///     }
///
///     pub fn turn_on(&mut self) {
///         self.on = true;
///     }
///
///     pub fn turn_off(&mut self) {
///         self.on = false;
///     }
/// }
///
/// impl RequestHandler for MyDevice {
///     fn property_request_received(&mut self, deoj: &mut Object, esv: Esv, prop: &Property) -> bool {
///         // Ignore all messages to other objects in the same node.
///         if deoj.code() != self.device.code() {
///             return false;
///         }
///
///         match esv {
///             Esv::WriteRequest | Esv::WriteReadRequest => {
///                 let prop_code = prop.code();
///                 let prop_bytes = prop.data();
///                 match prop_code {
///                     0x80 /* Operating status */ => {
///                         let prop_u32 = Bytes::to_u32(prop_bytes);
///                         match prop_u32 {
///                             0x30 /* On */=> {
///                                 self.turn_on();
///                                 return true;
///                             }
///                             0x31 /* Off */=> {
///                                 self.turn_off();
///                                 return true;
///                             }
///                             _ => {}
///                         }
///                     }
///                     _ => {}
///                 }
///             }
///             _ => {}
///         }
///         false
///     }
/// }
// ```

pub struct Device {
    code: ObjectCode,
    node: Arc<Mutex<DeviceNode>>,
}

impl Device {
    /// Create a new device.
    pub fn new(code: ObjectCode) -> Device {
        let mut dev = Device {
            code: code,
            node: DeviceNode::new(),
        };
        dev.init(code);
        dev
    }

    /// Create a new device with the node to which it belongs.
    pub fn new_with_node(code: ObjectCode, node: Arc<Mutex<Node>>) -> Device {
        let mut dev = Device {
            code: code,
            node: DeviceNode::new_with_node(node),
        };
        dev.init(code);
        dev
    }

    fn init(&mut self, code: ObjectCode) {
        let mut obj = Object::new();
        obj.set_code(code);
        obj.add_standard_properties(SUPER_OBJECT_CODE);
        obj.add_standard_properties(code);
        let mut dev_node = self.node.lock().unwrap();
        dev_node.add_object(obj);

        // Sets mandatory properties

        let vers: [u8; 4] = [0, 0, DEVICE_DEFAULT_VERSION_APPENDIX, 0];
        dev_node.set_property(self.code, DEVICE_STANDARD_VERSION, &vers);
        dev_node.set_property(
            self.code,
            DEVICE_MANUFACTURER_FAULT_CODE,
            &[DEVICE_NO_FAULT_OCCURRED],
        );
        dev_node.set_property(
            self.code,
            DEVICE_INSTALLATION_LOCATION,
            &[DEVICE_INSTALLATION_LOCATION_UNKNOWN],
        );
        dev_node.set_property(
            self.code,
            DEVICE_OPERATING_STATUS,
            &[OBJECT_OPERATING_STATUS_OFF],
        );
        let mut code: [u8; 3] = [0; 3];
        Bytes::from_u32(DEVICE_MANUFACTURER_EXPERIMENT, &mut code);
        dev_node.set_property(self.code, DEVICE_MANUFACTURER_CODE, &code);
    }

    /// Returns the object code.
    pub fn code(&self) -> ObjectCode {
        self.code
    }

    /// Returns the parent local node to which the device belongs.
    pub fn node(&self) -> Arc<Mutex<Node>> {
        let dev_node = self.node.lock().unwrap();
        dev_node.node()
    }

    // Returns the parent local node to which the device belongs.
    // rustc --explain E0515
    // pub fn object(&self) -> &mut Object {
    //     let mut node = self.node().lock().unwrap();
    //     let obj = node.find_object_mut(self.code);
    //     obj.unwrap()
    // }

    /// Sets the data into the specified property if the device node has the property, otherwise return false.
    pub fn set_property(&mut self, code: PropertyCode, data: &[u8]) -> bool {
        let mut dev_node = self.node.lock().unwrap();
        dev_node.set_property(self.code, code, data)
    }

    /// Gets the specified property data if the device node has the property, otherwise return none.
    pub fn property(&self, code: PropertyCode) -> Option<Vec<u8>> {
        let dev_node = self.node.lock().unwrap();
        dev_node.property(self.code, code)
    }

    pub fn set_request_handler(&mut self, handler: RequestHandlerObject) {
        let mut dev_node = self.node.lock().unwrap();
        dev_node.set_request_handler(handler.clone());
    }

    /// Starts the device to communicate with other ECHONET-Lite nodes on the local network.
    pub fn start(&mut self) -> bool {
        let mut dev_node = self.node.lock().unwrap();
        if !dev_node.start() {
            return false;
        }
        true
    }

    /// Stops the device.
    pub fn stop(&mut self) -> bool {
        let mut dev_node = self.node.lock().unwrap();
        dev_node.stop()
    }
}

impl Device {
    pub fn set_operating_status(&mut self, status: bool) -> bool {
        let status_byte: u8 = if status {
            OBJECT_OPERATING_STATUS_ON
        } else {
            OBJECT_OPERATING_STATUS_OFF
        };
        self.set_property(DEVICE_OPERATING_STATUS, &[status_byte])
    }

    pub fn operating_status(&mut self) -> Option<Vec<u8>> {
        self.property(DEVICE_OPERATING_STATUS)
    }

    pub fn set_installation_location(&mut self, loc: u8) -> bool {
        self.set_property(DEVICE_INSTALLATION_LOCATION, &[loc])
    }

    pub fn installation_location(&mut self) -> Option<Vec<u8>> {
        self.property(DEVICE_INSTALLATION_LOCATION)
    }

    pub fn set_standard_version(&mut self, ver: u8) -> bool {
        let mut vers: [u8; 4] = [0; 4];
        vers[2] = ver;
        self.set_property(DEVICE_STANDARD_VERSION, &vers)
    }

    pub fn standard_version(&mut self) -> Option<Vec<u8>> {
        self.property(DEVICE_STANDARD_VERSION)
    }

    pub fn set_fault_status(&mut self, status: bool) -> bool {
        let status_byte: u8 = if status {
            DEVICE_FAULT_OCCURRED
        } else {
            DEVICE_NO_FAULT_OCCURRED
        };
        self.set_property(DEVICE_FAULT_STATUS, &[status_byte])
    }

    pub fn fault_status(&mut self) -> Option<Vec<u8>> {
        self.property(DEVICE_FAULT_STATUS)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        self.stop();
    }
}
