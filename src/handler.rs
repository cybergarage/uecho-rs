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

use crate::object::ObjectCode;
use crate::protocol::{Esv, Message, Property};
use std::sync::{Arc, Mutex};

/// RequestHandler defines a request message handler interface.
pub trait RequestHandler {
    fn property_request_received(&mut self, deoj: ObjectCode, esv: Esv, prop: &Property) -> bool;
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

    pub fn notify(&self, msg: &Message) -> bool {
        let deoj = msg.deoj();
        let esv = msg.esv();
        for handler in self.handlers.iter() {
            for prop in msg.properties() {
                handler
                    .lock()
                    .unwrap()
                    .property_request_received(deoj, esv, prop);
            }
        }
        true
    }
}
