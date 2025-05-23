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

use log::*;
use std::sync::Arc;
use std::sync::Mutex;

use echonet::{Controller, Device, Node, Object, ObjectCode, RequestHandler};

use echonet::protocol::{Property, ESV};
use echonet::util::Bytes;

pub struct TestController {}

impl TestController {
    pub fn new(node: Arc<Mutex<Node>>) -> Controller {
        Controller::new_with_node(node)
    }
}

pub struct TestDevice {
    dev: Device,
    num_on_req: u32,
    num_off_req: u32,
}

impl TestDevice {
    pub fn new(node: Arc<Mutex<Node>>) -> Arc<Mutex<TestDevice>> {
        let m = Arc::new(Mutex::new(TestDevice {
            dev: Device::new_with_node(0x05FD, node), // Switch (supporting JEM-A/HA terminals)
            num_on_req: 0,
            num_off_req: 0,
        }));
        m.lock().unwrap().dev.set_request_handler(m.clone());
        m
    }

    pub fn code(&self) -> ObjectCode {
        self.dev.code()
    }

    pub fn start(&mut self) -> bool {
        self.dev.start()
    }

    pub fn stop(&mut self) -> bool {
        self.dev.stop()
    }
}

impl RequestHandler for TestDevice {
    fn property_request_received(&mut self, deoj: &mut Object, esv: ESV, prop: &Property) -> bool {
        // Ignore all messages to other objects in the same node.
        if deoj.code() != self.dev.code() {
            return false;
        }

        match esv {
            ESV::WriteRequest | ESV::WriteReadRequest | ESV::WriteRequestResponseRequired => {
                let prop_code = prop.code();
                let prop_bytes = prop.data();
                match prop_code {
                    0x80 /* Operating status */ => {
                        let prop_u32 = Bytes::to_u32(prop_bytes);
                        match prop_u32 {
                            0x30 /* On */=> {
                                info!("On");
                                deoj.set_property_byte(prop_code, 0x30);
                                self.num_on_req+= 1;
                            }
                            0x31 /* Off */=> {
                                info!("Off");
                                deoj.set_property_byte(prop_code, 0x31);
                                self.num_off_req+= 1;
                            }
                            _ => {
                                return false;
                            }
                        }
                    }
                    _ => {
                        return false;
                    }
                }
            }
            _ => {}
        }

        true
    }
}
