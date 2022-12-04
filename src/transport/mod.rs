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

pub use self::default::PORT;
pub use self::manager::Manager;
pub use self::notify_manager::NotifytManager;
pub use self::observer::{Observer, ObserverObject};

mod default;
mod error;
mod interface;
mod manager;
mod multicast_manager;
mod multicast_server;
mod notifier;
mod notify_manager;
mod observer;
mod result;
mod udp_socket;
mod unicast_manager;
mod unicast_server;

mod interface_test;
mod manager_test;
mod multicast_manager_test;
mod multicast_server_test;
mod notify_manager_test;
mod unicast_manager_test;
mod unicast_server_test;
