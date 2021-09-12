// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::transport::manager::*;

    #[test]
    fn manager_test() {
        let mut mgr = Manager::new();
        assert!(mgr.start());
        assert!(mgr.stop());
    }
}
