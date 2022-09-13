// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use pnet::datalink;
use pnet::ipnetwork;
use std::net::IpAddr;

type EnableInterface = fn(ipnetwork::IpNetwork) -> bool;

fn is_all_interface(_ipnet: ipnetwork::IpNetwork) -> bool {
    true
}

fn is_v4_interface(ipnet: ipnetwork::IpNetwork) -> bool {
    ipnet.is_ipv4()
}

fn is_v6_interface(ipnet: ipnetwork::IpNetwork) -> bool {
    ipnet.is_ipv6()
}

fn get_interfaces(enable_interface: EnableInterface) -> Vec<IpAddr> {
    let mut ifaddrs = Vec::new();
    for iface in datalink::interfaces() {
        if iface.is_loopback() || iface.ips.is_empty() {
            continue;
        }
        for ifaddr in iface.ips {
            if !enable_interface(ifaddr) {
                continue;
            }
            ifaddrs.push(ifaddr.ip());
        }
    }
    ifaddrs
}

pub fn get_all_interfaces() -> Vec<IpAddr> {
    get_interfaces(is_all_interface)
}

pub fn get_v4_interfaces() -> Vec<IpAddr> {
    get_interfaces(is_v4_interface)
}

pub fn get_v6_interfaces() -> Vec<IpAddr> {
    get_interfaces(is_v6_interface)
}
