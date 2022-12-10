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
//! The `uecho-rs` provides the following major components to control [ECHONET-Lite][enet] devices and create the standard specification devices.
//!
//! - [ECHONET-Lite][enet] controller to find and control [ECHONET-Lite][enet] nodes.
//! - [ECHONET-Lite][enet] device framework to implement any standard [ECHONET-Lite][enet] devices.
//! - Decoder and Encoder for [ECHONET-Lite][enet] messaging packet.
//! - Standard device database based on [Machine Readable Appendix][mra] and [Manufacturer code List][mcl] provided by [the ECHONET Consortium][eneto].
//!
//! The `uecho-rs` handles all [ECHONET-Lite][enet] requests such as read, write and notification requests automatically. Therefore, Rust developers can implement the standard device and controller applications using the `uecho-rs` easily by simply implementing the request message validators.
//!
//! ## Getting Started
//!
//! The examples are available for [ECHONET-Lite][enet] controller and device implementations using the `uecho-rs`, check out [the examples folder in GitHub](https://github.com/cybergarage/uecho-rs/tree/master/examples).
//!
//! ## Table of Contents
//!
//! - Controller
//!   - [Overview of Controller](https://github.com/cybergarage/uecho-rs/blob/master/doc/controller_overview.md)
//! - Device
//!   - [Overview of Device](https://github.com/cybergarage/uecho-rs/blob/master/doc/device_overview.md)
//!   - [Inside of Device](https://github.com/cybergarage/uecho-rs/blob/master/doc/device_inside.md)
//! - Examples
//!   - [Usage examples](https://github.com/cybergarage/uecho-rs/tree/master/examples)
//!
//! ## Getting Help
//!
//! - [Generated Docs (latest version)](https://docs.rs/echonet/latest/echonet/)
//!
//! ## License
//!
//! This project is licensed under the Apache-2.0 License.
//!
//! [enet]:http://echonet.jp/english/
//! [eneto]:https://echonet.jp/organization_en/
//! [mra]:https://echonet.jp/spec_mra_rp1_en/
//! [mcl]:https://echonet.jp/spec-en/

/// Logger function module.
pub mod log;
/// messaging packet encoder and decoder module.
pub mod protocol;
/// Messaging transport manager module (Internal).
pub mod transport;
/// Utility function module.
pub mod util;

pub use self::controller::*;
pub use self::database::*;
pub use self::device::Device;
pub use self::manufacture::{Manufacture, ManufactureCode};
pub use self::node::*;
pub use self::object::{Object, ObjectCode};
pub use self::property::{Property, PropertyEnum, PropertyRule};
pub use self::remote_node::*;
pub use self::request_handler::{RequestHandler, RequestHandlerObject};

mod class;
mod controller;
mod controller_node;
mod database;
mod database_manufacturers;
mod database_mra_objects;
mod device;
mod device_node;
mod error;
mod manufacture;
mod message;
mod node;
mod node_profile;
mod object;
mod profile;
mod property;
mod remote_node;
mod request_handler;
mod super_object;

mod controller_test;
mod database_test;
mod device_test;
mod message_test;
mod node_test;
mod object_test;
mod profile_test;
mod property_test;
mod remote_node_test;
mod node_profile_test;
