// Copyright (C) 2022 Satoshi Konno All rights reserved.
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

#[cfg(test)]
mod tests {

    use crate::transport::interface::*;

    #[test]
    fn v4_interface() {
        let ifaddrs = get_all_interfaces();
        for ifaddr in ifaddrs {
            assert!(ifaddr.is_ipv4())
        }
    }

    #[test]
    fn v6_interface() {
        let ifaddrs = get_v6_interfaces();
        for ifaddr in ifaddrs {
            assert!(ifaddr.is_ipv6())
        }
    }

    #[test]
    fn all_interface() {
        let ifaddrs = get_all_interfaces();
        for ifaddr in ifaddrs {
            assert!(ifaddr.is_ipv6() || ifaddr.is_ipv4())
        }
    }
}
