// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::controller::*;

    #[test]
    fn local_node_test() {
        let mut ctrl = Controller::new();
        ctrl.add_observer(Arc::new(Mutex::new(ctrl));
        ctrl.start();
        ctrl.stop();
    }
}
