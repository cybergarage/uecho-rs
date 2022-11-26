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

#![allow(dead_code)]

use pnet::datalink;
use pnet::ipnetwork;
use std::net::IpAddr;

type EnableInterface = fn(ipnetwork::IpNetwork) -> bool;

fn is_ignore_interface(ipnet: ipnetwork::IpNetwork) -> bool {
    let binding = ipnet.to_string();
    let ifaddr = binding.as_str();
    match ifaddr {
        "172.17.0.1/16" => return true, // Docker default gateway
        _ => return false,
    }
}

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
        if !iface.is_up() {
            continue;
        }
        if iface.is_loopback() || iface.is_point_to_point() {
            continue;
        }
        if iface.ips.is_empty() {
            continue;
        }
        for ifaddr in iface.ips {
            if is_ignore_interface(ifaddr) {
                continue;
            }
            if !enable_interface(ifaddr) {
                continue;
            }
            ifaddrs.push(ifaddr.ip());
        }
    }
    ifaddrs
}

pub fn get_all_interfaces() -> Vec<IpAddr> {
    get_interfaces(is_v4_interface)
}

pub fn get_v4_interfaces() -> Vec<IpAddr> {
    get_interfaces(is_v4_interface)
}

pub fn get_v6_interfaces() -> Vec<IpAddr> {
    get_interfaces(is_v6_interface)
}
