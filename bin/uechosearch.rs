// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use uecho::controller::*;
use uecho::log::*;

fn main() {
    logger::init();

    let mut ctrl = Controller::new();
    ctrl.start();
    ctrl.search_all();
    ctrl.stop();
}
