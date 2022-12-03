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

use std::env;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use echonet::log::Logger;
use echonet::protocol::{Esv, Property};
use echonet::util::Bytes;
use echonet::{Device, ObjectCode, RequestHandler};
use log::Log;

/// MonoLight represents a mono functional lighting device of a Echonet-Lite standard devide.
pub struct MonoLight {
    device: Device,
}

impl MonoLight {
    pub fn new() -> Arc<Mutex<MonoLight>> {
        let m = Arc::new(Mutex::new(MonoLight {
            device: Device::new(0x029101),
        }));
        m.lock().unwrap().device.set_request_handler(m.clone());
        m
    }

    fn start(&mut self) -> bool {
        self.device.start()
    }

    fn stop(&mut self) -> bool {
        self.device.stop()
    }
}

impl RequestHandler for MonoLight {
    fn property_request_received(&mut self, deoj: ObjectCode, esv: Esv, prop: &Property) -> bool {
        // Ignore all messages to other objects in the same node.
        if deoj != self.device.code() {
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
                                self.device.set_property(prop_code, prop_bytes);
                            }
                            0x31 /* Off */=> {
                                self.device.set_property(prop_code, prop_bytes);
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

fn main() -> Result<(), Error> {
    Logger::init();
    for arg in env::args() {
        print!("{}", arg);
        match arg.as_str() {
            "-v" => {
                Logger::init();
            }
            &_ => {}
        }
    }

    let ml = MonoLight::new();
    let mut ml = ml.lock().unwrap();
    ml.start();

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        thread::sleep(time::Duration::from_secs(1));
    }

    ml.stop();

    Ok(())
}
