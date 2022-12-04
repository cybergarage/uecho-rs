![logo](doc/img/logo.png)

[![crates.io](https://img.shields.io/crates/v/echonet.svg)](https://crates.io/crates/echonet)
[![crates.io](https://img.shields.io/crates/d/echonet?label=cargo%20installs)](https://crates.io/crates/echonet)
[![cargo-test](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml)
[![docs.rs](https://img.shields.io/badge/Rustdoc-docs.rs-blueviolet)](https://docs.rs/echonet)

The `uecho-rs` is a portable, cross-platform development framework for Rust developers to create [ECHONET-Lite][enet] controller and device applications. [ECHONET-Lite][enet] is an open standard specification for IoT devices in Japan, it specifies more than 100 IoT devices such as crime prevention sensors, air conditioners and refrigerators.

## What is uEcho ?

To implement IoT controllers or devices of [ECHONET-Lite][enet], the developer had to understand and implement the communication middleware specification such as the message format and base sequences.

The `uecho-rs` supports to control devices of [ECHONET-Lite][enet] or create the standard devices of the specification easily. 

![](doc/img/framework.png)

Using the `uecho-rs`, the developer has only to set basic listeners to implement the devices and controllers because the `uecho-rs` handles other requests such as read and notification requests automatically.

## Getting Started

To add the `uecho-rs` to your project, add the following to your Cargo.toml file:

```
[dependencies]
echonet = "1.x.x"
```

The examples are available for [ECHONET-Lite][enet] controller and device implementations using the `uecho-rs`, check out [the examples folder in GitHub](https://github.com/cybergarage/uecho-rs/tree/master/examples).

## Table of Contents

- Controller
  - [Overview of Controller](https://github.com/cybergarage/uecho-rs/blob/master/doc/controller_overview.md)
- Device
  - [Overview of Device](https://github.com/cybergarage/uecho-rs/blob/master/doc/device_overview.md)
  - [Inside of Device](https://github.com/cybergarage/uecho-rs/blob/master/doc/device_inside.md)
- Examples
  - [Usage examples](https://github.com/cybergarage/uecho-rs/tree/master/examples)

## Getting Help

- [crates.io: Rust Package Registry - echonet(latest version)](https://crates.io/crates/echonet)
- [Generated Docs (latest version)](https://docs.rs/echonet/latest/echonet/)

## License

This project is licensed under the Apache-2.0 License.

[enet]:http://echonet.jp/english/

