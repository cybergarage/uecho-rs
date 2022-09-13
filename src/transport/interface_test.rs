// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::transport::interface::*;

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

    #[test]
    fn all_interface_test() {
        let ifaddrs = get_all_interfaces();
        for ifaddr in ifaddrs {
            assert!(ifaddr.is_ipv6() || ifaddr.is_ipv4())
        }
    }
}
