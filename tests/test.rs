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

use echonet::{Controller, Device, LocalNode, ObjectCode, RequestHandler};

use echonet::protocol::{Esv, Property};
use echonet::util::Bytes;

pub struct TestController {}

impl TestController {
    pub fn new(node: Arc<Mutex<LocalNode>>) -> Controller {
        Controller::new_with_node(node)
    }
}

pub struct TestDevice {
    dev: Device,
    num_on_req: u32,
    num_off_req: u32,
}

impl TestDevice {
    pub fn new(node: Arc<Mutex<LocalNode>>) -> Arc<Mutex<TestDevice>> {
        let m = Arc::new(Mutex::new(TestDevice {
            dev: Device::new_with_node(0x05FD, node), // Switch (supporting JEM-A/HA terminals)
            num_on_req: 0,
            num_off_req: 0,
        }));
        m.lock().unwrap().dev.set_request_handler(m.clone());
        m
    }

    pub fn start(&mut self) -> bool {
        self.dev.start()
    }

    pub fn stop(&mut self) -> bool {
        self.dev.stop()
    }
}

impl RequestHandler for TestDevice {
    fn property_request_received(&mut self, deoj: ObjectCode, esv: Esv, prop: &Property) -> bool {
        // Ignore all messages to other objects in the same node.
        if deoj != self.dev.code() {
            return false;
        }

        match esv {
            Esv::WriteRequest | Esv::WriteReadRequest => {
                let prop_code = prop.code();
                let prop_bytes = prop.data();
                match prop_code {
                    0x80 /* Operating status */ => {
                        let prop_u32 = Bytes::to_u32(prop_bytes);
                        match prop_u32 {
                            0x30 /* On */=> {
                                self.num_on_req+= 1;
                                self.dev.set_property(prop_code, prop_bytes);
                            }
                            0x31 /* Off */=> {
                                self.num_off_req+= 1;
                                self.dev.set_property(prop_code, prop_bytes);
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
