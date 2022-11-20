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

use std::net::{Ipv4Addr, Ipv6Addr};

pub const MULTICAST_V4_ADDRESS: Ipv4Addr = Ipv4Addr::new(224, 0, 23, 0);
pub const MULTICAST_V6_ADDRESS: Ipv6Addr = Ipv6Addr::new(0xff, 0x02, 0, 0, 0, 0, 0, 1);
pub const PORT: u16 = 3610;
pub const MAX_PACKET_SIZE: usize = 1024;
