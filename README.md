![logo](doc/img/logo.png)

[![crates.io](https://img.shields.io/crates/v/echonet.svg)](https://crates.io/crates/echonet)
[![crates.io](https://img.shields.io/crates/d/echonet?label=cargo%20installs)](https://crates.io/crates/echonet)
[![cargo-test](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml)
[![docs.rs](https://img.shields.io/badge/Rustdoc-docs.rs-blueviolet)](https://docs.rs/echonet)

The `uecho-rs` is a portable, cross-platform development framework for Rust developers to create [ECHONET-Lite][enet] controller and device applications. [ECHONET-Lite][enet] is an open standard specification for IoT devices in Japan, it specifies more than 100 IoT devices such as crime prevention sensors, air conditioners and refrigerators.

## What is uEcho ?

The `uecho-rs` is a comprehensive framework for [ECHONET-Lite][enet] development in Rust. To implement IoT controllers or devices of [ECHONET-Lite][enet], the developer had to understand and implement the communication middleware specification such as the message format and base sequences.

The `uecho-rs` provides the following major components to control [ECHONET-Lite][enet] devices and create the standard [ECHONET-Lite][enet] devices easily without in-depth [ECHONET-Lite][enet] specification understanding. 

- [ECHONET-Lite][enet] controller to find and control [ECHONET-Lite][enet] nodes.
- [ECHONET-Lite][enet] device framework to implement any standard [ECHONET-Lite][enet] devices.
- Decoder and Encoder for [ECHONET-Lite][enet] messaging packet.
- Standard device database based on [Machine Readable Appendix][mra] and [Manufacturer code List][mcl] provided by [the ECHONET Consortium][eneto].

The `uecho-rs` handles all [ECHONET-Lite][enet] requests such as read, write and notification requests automatically. Therefore, Rust developers can implement the standard device and controller applications using the `uecho-rs` easily by simply implementing the request message validators.

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
  - [Usage examples (For Raspberry Pi Sense HAT)](https://github.com/cybergarage/uecho-rs-sensehat)

## Getting Help

- [crates.io: Rust Package Registry - echonet(latest version)](https://crates.io/crates/echonet)
- [Generated Docs (latest version)](https://docs.rs/echonet/latest/echonet/)

## License

This project is licensed under the Apache-2.0 License.

[enet]:http://echonet.jp/english/
[eneto]:https://echonet.jp/organization_en/
[mra]:https://echonet.jp/spec_mra_rp1_en/
[mcl]:https://echonet.jp/spec-en/
