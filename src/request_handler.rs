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

use crate::object::Object;
use crate::protocol::{Message, Property, ESV};
use std::sync::{Arc, Mutex};

/// RequestHandler defines a request message handler interface.
/// # Examples
/// ```
/// use std::sync::{Arc, Mutex};
/// use echonet::{Device, ObjectCode, Object, RequestHandler};
/// use echonet::protocol::{ESV, Property};
/// use echonet::util::Bytes;
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
///     fn property_request_received(&mut self, deoj: &mut Object, esv: ESV, prop: &Property) -> bool {
///         match esv {
///             ESV::WriteRequest | ESV::WriteReadRequest => {
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
///                             _ => {
///                             }
///                         }
///                     }
///                     _ => {
///                     }
///                 }
///             }
///             _ => {}
///         }
///         false
///     }
/// }
/// ```

pub trait RequestHandler {
    fn property_request_received(&mut self, deoj: &mut Object, esv: ESV, prop: &Property) -> bool;
}

/// RequestHandlerObject represents a request message handler object.
pub type RequestHandlerObject = Arc<Mutex<dyn RequestHandler + Send>>;

pub type RequestHandlers = Vec<RequestHandlerObject>;

pub struct RequestManager {
    handlers: RequestHandlers,
}

impl RequestManager {
    pub fn new() -> RequestManager {
        RequestManager {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, observer: RequestHandlerObject) -> bool {
        self.handlers.push(observer);
        true
    }

    pub fn property_request_received(&mut self, deoj: &mut Object, msg: &Message) -> bool {
        if self.handlers.len() == 0 {
            return true;
        }

        let esv = msg.esv();
        let mut request_accepted = false;
        for handler in self.handlers.iter() {
            let mut all_requests_accepted = true;
            for prop in msg.properties() {
                if !handler
                    .lock()
                    .unwrap()
                    .property_request_received(deoj, esv, prop)
                {
                    all_requests_accepted = false;
                }
            }
            if all_requests_accepted {
                request_accepted = true
            }
        }
        request_accepted
    }
}
