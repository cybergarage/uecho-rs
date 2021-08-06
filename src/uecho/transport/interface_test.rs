// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::transport::interface::*;

    #[test]
    fn v4_interface_test() {
        let ifaddrs = get_v4_interfaces();
        for ifaddr in ifaddrs {
            assert!(ifaddr.is_ipv4())
        }
    }

    #[test]
    fn v6_interface_test() {
        let ifaddrs = get_v6_interfaces();
        for ifaddr in ifaddrs {
            assert!(ifaddr.is_ipv6())
        }
    }
}
