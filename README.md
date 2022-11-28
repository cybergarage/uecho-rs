![logo](doc/img/logo.png)

![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/cybergarage/uecho-rs)
[![cargo](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/badge/Rust-crates.io-orange)](https://crates.io/crates/echonet)
[![docs.rs](https://img.shields.io/badge/Rust-document-blue)](https://docs.rs/echonet/latest/echonet/)

The `uecho-rs` is a portable and cross-platform development framework for creating controller applications and devices of [ECHONET-Lite][enet] for Rust developers. [ECHONET-Lite][enet] is an open standard specification for IoT devices in Japan, it specifies more than 100 IoT devices such as crime prevention sensors, air conditioners and refrigerators.

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

## Getting Help

- [echonet - crates.io: Rust Package Registry (latest version)](https://crates.io/crates/echonet)
- [Generated Docs (latest version)](https://docs.rs/echonet/latest/echonet/)
- [Usage examples](https://github.com/cybergarage/uecho-rs/tree/master/examples)

## License

This project is licensed under the Apache-2.0 License.

[enet]:http://echonet.jp/english/

