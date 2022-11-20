// Copyright (C) 2022 The uecho-rs Authors All rights reserved.
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

pub mod default;
pub mod error;
pub mod interface;
pub mod manager;
pub mod multicast_manager;
pub mod multicast_server;
pub mod notifier;
pub mod notify_manager;
pub mod observer;
pub mod result;
pub mod udp_socket;
pub mod unicast_manager;
pub mod unicast_server;

mod interface_test;
mod manager_test;
mod multicast_manager_test;
mod multicast_server_test;
mod notify_manager_test;
mod unicast_manager_test;
mod unicast_server_test;
