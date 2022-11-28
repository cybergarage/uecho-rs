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

use echonet::protocol::{Esv, Property};
use echonet::{Device, ObjectCode, RequestHandler};

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
        true
    }
}

fn main() {
    let ml = MonoLight::new();
    ml.lock().unwrap().start();
    ml.lock().unwrap().stop();
}