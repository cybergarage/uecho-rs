![logo](doc/img/logo.png)

![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/cybergarage/uecho-rs)
[![cargo](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/cybergarage/uecho-rs/actions/workflows/cargo.yml)
[![docs.rs](https://img.shields.io/badge/Rust-document-blue)](https://docs.rs/echonet/latest/echonet/)

The `uecho-rs` is a portable and cross-platform development framework for creating controller applications and devices of [ECHONET-Lite][enet] for Rust developers. [ECHONET-Lite][enet] is an open standard specification for IoT devices in Japan, it specifies more than 100 IoT devices such as crime prevention sensors, air conditioners and refrigerators.

## What is uEcho ?

The `uecho-rs` supports to control devices of [ECHONET-Lite][enet] or create the standard devices of the specification easily. To implement IoT controllers or devices of [ECHONET-Lite][enet], the developer had to understand and implement the communication middleware specification such as the message format and base sequences.

![](doc/img/framework.png)

However, the `uecho-rs`, developer has only to set basic listeners to implement the devices and controllers because uEcho handles other requests such as request and notification requests automatically.

[enet]:http://echonet.jp/english/
