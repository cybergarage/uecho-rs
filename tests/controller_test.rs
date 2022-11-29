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

use std::{thread, time};

use echonet::log::logger;
use echonet::LocalNode;

mod test;

#[test]
fn controller() {
    logger::init();

    let node = LocalNode::new();

    let mut dev = test::TestDevice::new(node.clone());
    assert!(dev.lock().unwrap().start());

    let mut ctrl = test::TestController::new(node.clone());
    assert!(ctrl.start());
    assert!(ctrl.search());
    thread::sleep(time::Duration::from_secs(0));
    assert!(ctrl.stop());

    assert!(dev.lock().unwrap().stop());
}
