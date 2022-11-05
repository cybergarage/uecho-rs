![logo](doc/img/logo.png)

The `uecho-rs` is a portable and cross platform development framework for creating controller applications and devices of [ECHONET Lite][enet] for Rust developers. [ECHONET][enet] is an open standard specification for IoT devices in Japan, it specifies more than 100 IoT devices such as crime prevention sensor, air conditioner and refrigerator.

## What is uEcho ?

The `uecho-rs` supports to control devices of [ECHONET Lite][enet] or create the standard devices of the specification easily. The `uecho-rs` is designed in object-oriented programming, and the functions are object-oriented in their naming convention, and are grouped into classes such as `Controller`, `Node`, `Class` and `Object`.

![](doc/img/framework.png)

To implement IoT controller or devices of [ECHONET Lite][enet], the developer had to understand and implement the communication middleware specification such as the message format and base sequences.

The `uecho-rs` is inspired by reactive programming too. Using The `uecho-rs`, developer have only to set basic listeners to implement the devices and controllers because uEcho handles other requests such as request and notification requests automatically.

[enet]:http://echonet.jp/english/
