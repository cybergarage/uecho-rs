![logo](doc/img/logo.png)

[![crates.io](https://img.shields.io/crates/v/echonet.svg)](https://crates.io/crates/echonet)
[![crates.io](https://img.shields.io/crates/d/echonet?label=cargo%20installs)](https://crates.io/crates/echonet)
[![cargo-test](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml)
[![docs.rs](https://img.shields.io/badge/Rustdoc-docs.rs-blueviolet)](https://docs.rs/echonet)

`uecho-rs` is a portable, cross-platform framework for developing [ECHONET Lite][enet] controllers and devices in Rust. ECHONET Lite is an open standard for IoT devices in Japan. It defines more than 100 device types, including security sensors, air conditioners, and refrigerators.

## What is uEcho?

Implementing ECHONET Lite controllers and devices from scratch requires handling protocol details such as message formats and communication sequences.

`uecho-rs` provides the following components to help developers build controllers and device applications:

- A controller for discovering and controlling ECHONET Lite nodes.
- A framework for implementing ECHONET Lite device applications.
- An encoder and decoder for ECHONET Lite messages.
- A standard device database based on [Machine Readable Appendix][mra] and [Manufacturer Code List][mcl] provided by [the ECHONET Consortium][eneto].

`uecho-rs` handles read/write requests and notifications. Developers can customize how their device applications validate request messages.

## Getting Started

To use `uecho-rs`, add the `echonet` crate to your `Cargo.toml`:

```toml
[dependencies]
echonet = "1"
```

The current source requires Rust 1.90 or later (edition 2024). The default feature is `unix`, which enables the standard-library networking implementation. Feature flags also include `win`, `no_std`, and `esp`; these are platform configuration options, not a guarantee that every target or dependency works without the standard library. In particular, adding `no_std` alone does not disable the default `unix` feature. Inspect [`Cargo.toml`](Cargo.toml) and [`src/transport/`](src/transport/) when targeting another platform.

### Try the tools and example

```sh
git clone https://github.com/cybergarage/uecho-rs.git
cd uecho-rs
cargo build
cargo run --bin uechoctl -- --help
cargo run --bin uechoctl -- scan
```

`scan` discovers nodes and reads properties. Discovery requires a network interface that can exchange ECHONET Lite UDP multicast traffic with the target devices. An empty scan can mean there are no reachable devices on that network.

To run a sample mono-functional lighting device:

```sh
cargo run --example monolight
```

The [example source](examples/monolight/monolight.rs) shows how a `Device` and a `RequestHandler` implement application-specific property handling. Stop it with Ctrl-C.

## Architecture and scope

The [`protocol`](src/protocol/) module encodes and decodes messages, while [`transport`](src/transport/) handles network communication. Controllers discover remote nodes and send requests; device applications expose objects and properties and implement request handlers. The standard database supplies object and manufacturer metadata.

Applications remain responsible for device behavior and property validation. Included metadata and example profiles do not establish certification or interoperability with every ECHONET Lite device. Test against your target devices and network.

## Development and testing

```sh
cargo build --examples
cargo test -- --test-threads=1
cargo doc --no-deps
```

Tests are serialized as in the repository Makefile. Network-dependent tests need suitable local interfaces and multicast access. Use direct Cargo commands if you want to avoid the automatic formatting performed by `make build` and `make test`.

## Table of Contents

- Controller
  - [Overview of Controller](https://github.com/cybergarage/uecho-rs/blob/master/doc/controller_overview.md)
- Device
  - [Overview of Device](https://github.com/cybergarage/uecho-rs/blob/master/doc/device_overview.md)
  - [Inside of Device](https://github.com/cybergarage/uecho-rs/blob/master/doc/device_inside.md)
- Examples
  - [Usage examples](https://github.com/cybergarage/uecho-rs/tree/master/examples)
  - [Raspberry Pi Sense HAT examples](https://github.com/cybergarage/uecho-rs-sensehat)

## Related projects

[uecho-simulator](https://github.com/cybergarage/uecho-simulator) is a small ECHONET Lite development simulator with virtual lighting, air conditioning, and temperature sensing. It provides a full-screen terminal UI and a live, read-only browser preview, runs offline by default, and implements limited device profiles.

## Getting Help

- [echonet on crates.io](https://crates.io/crates/echonet)
- [API documentation (latest version)](https://docs.rs/echonet/latest/echonet/)

## License

This project is licensed under the Apache-2.0 License.

[enet]:https://echonet.jp/english/
[eneto]:https://echonet.jp/organization_en/
[mra]:https://echonet.jp/spec-en/#standard-08
[mcl]:https://echonet.jp/spec-en/#standard-07
