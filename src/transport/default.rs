// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::{Ipv4Addr, Ipv6Addr};

pub const MULTICAST_V4_ADDRESS: Ipv4Addr = Ipv4Addr::new(224, 0, 23, 0);
pub const MULTICAST_V6_ADDRESS: Ipv6Addr = Ipv6Addr::new(0xff, 0x02, 0, 0, 0, 0, 0, 1);
pub const PORT: u16 = 3610;
pub const MAX_PACKET_SIZE: usize = 1024;
