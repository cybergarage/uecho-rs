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

use uecho::controller::Controller;
use uecho::device::Device;
use uecho::local_node::LocalNode;

pub struct TestController {}

impl TestController {
    pub fn new(node: Arc<Mutex<LocalNode>>) -> Controller {
        Controller::new_with_node(node)
    }
}
pub struct TestDevice {}

impl TestDevice {
    pub fn new(node: Arc<Mutex<LocalNode>>) -> Device {
        Device::new_with_node(0x029101, node)
    }
}