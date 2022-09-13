// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::transport::multicast_manager::*;

    #[test]
    fn multicast_manager_test() {
        let mut mgr = MulticastManager::new();
        assert!(mgr.start());
        assert!(mgr.stop());
    }
}