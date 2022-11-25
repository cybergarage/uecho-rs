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

use uecho::local_node::LocalNode;
use uecho::log::logger;

mod mock;

#[test]
fn controller() {
    logger::init();

    let node = LocalNode::new();

    let _dev = mock::TestDevice::new(node.clone());
    // assert!(dev.start());

    let mut ctrl = mock::TestController::new(node.clone());
    assert!(ctrl.start());
    assert!(ctrl.search_all());
    thread::sleep(time::Duration::from_secs(1));
    assert!(ctrl.stop());

    // assert!(dev.sop());
}
