// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::controller::*;
    use crate::log::logger;

    #[test]
    fn controller_test() {
        logger::init();

        let mut ctrl = Controller::new();
        ctrl.add_observer(Arc::new(Mutex::new(ctrl));
        assert(ctrl.start());
        assert(ctrl.search_all());
        assert(ctrl.stop());
    }
}
