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

//! # echonet (uecho-rs)
//! The `uecho-rs` is a portable and cross-platform development framework for creating controller applications and devices of [ECHONET-Lite][enet] for Rust developers. [ECHONET-Lite][enet] is an open standard specification for IoT devices in Japan, it specifies more than 100 IoT devices such as crime prevention sensors, air conditioners and refrigerators.
//!
//! ## What is uEcho ?
//!
//! To implement IoT controllers or devices of [ECHONET-Lite][enet], the developer had to understand and implement the communication middleware specification such as the message format and base sequences.
//!
//! The `uecho-rs` supports to control devices of [ECHONET-Lite][enet] or create the standard devices of the specification easily. Using the `uecho-rs`, the developer has only to set basic listeners to implement the devices and controllers because the `uecho-rs` handles other requests such as read and notification requests automatically.
//!
//! ## Getting Started
//!
//! The examples are available for [ECHONET-Lite][enet] controller and device implementations using the `uecho-rs`, check out [the examples folder in GitHub](https://github.com/cybergarage/uecho-rs/tree/master/examples).
//!
//! ## Getting Help
//!
//! - [Generated Docs (latest version)](https://docs.rs/echonet/latest/echonet/)
//! - [Usage examples](https://github.com/cybergarage/uecho-rs/tree/master/examples)
//!
//! ## License
//!
//! This project is licensed under the Apache-2.0 License.
//!
//! [enet]:http://echonet.jp/english/

/// Logger function module.
pub mod log;
/// Messaging protocol encoder and decoder module.
pub mod protocol;
/// Messaging transport manager module (Internal).
pub mod transport;
/// Utility function module.
pub mod util;

pub use self::controller::*;
pub use self::database::*;
pub use self::device::Device;
pub use self::local_node::*;
pub use self::manufacture::{Manufacture, ManufactureCode};
pub use self::object::Object;
pub use self::property::{Property, PropertyEnum, PropertyRule};
pub use self::remote_node::*;

mod class;
mod controller;
mod controller_node;
mod database;
mod database_manufacturers;
mod database_mra_objects;
mod device;
mod device_node;
mod error;
mod local_node;
mod manufacture;
mod message;
mod node_profile;
mod object;
mod profile;
mod property;
mod remote_node;
mod super_object;

mod controller_test;
mod database_test;
mod device_test;
mod local_node_test;
mod message_test;
mod object_test;
mod profile_test;
mod property_test;
mod remote_node_test;
