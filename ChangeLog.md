# Changelog

## v1.5.x (2023-xx-xx)
- Support ESP32 platforms
- Update transport layer
- Enable IPv6 interfaces
- Support Supports multi-network interfaces

## v1.4.x (2023-xx-xx)
- Support ESP32 platforms

## v1.3.x (2023-xx-xx)
- Improve controller search functions
- Improve uechosearch example to output object properties using the set property map
- Improve uechosearch example to output more human readable properties using the standard database

## v1.3.2 (2023-xx-xx)
- Remove once_cell

## v1.3.1 (2023-09-02)
- Add no_std keyword

## v1.3.0 (2023-08-23)
- Update for Rust 1.7
- Add std and no_std features
 
## 1.2.7 (2023-05-07)
- Updated the standard manufacturer code database based on the latest MCA (Manufacturer Code List) from the ECHONET Consortium
- Updated the standard object database based on the latest MRA (Machine Readable Appendix) version 1.2.0 from the ECHONET Consortium

## v1.2.6 (2023-02-11)
- Remove dev-dependency packages

## v1.2.5 (2023-01-20)
- Update Controller::send_message() to try lock

## v1.2.4 (2023-01-05)
- Add uechobench for benchmarking

## v1.2.3 (2022-12-23)
- Update UdpSocket::bind() to retry for repeated binding errors
- Update UdpSocket::close() to add sleep wait for epeated binding errors

## v1.2.2 (2022-12-18)
- Improve Device to set mandatory properties

## v1.2.1 (2022-12-10)
- Update standard device objects to add extra devices and definitions
- Update standard manufactuer codes
- Update uechopost not to wait when the specified request message does not require the response message
- Improve standard objects to set more default standard properties
- Add a multiple device example on Raspberry Pi Sense HAT

## v1.2.0 (2022-12-09)
- Update RequestHandler to pass a mutable destination object to write and return a latest property data for read reuests data by the request handler
- Updated Device::new() to output warnings when the standart object is not found

## v1.1.0 (2022-12-07)
-  Supported write and read request protocols (0x6E, 0x7E, 0x5E)

## v1.0.0 (2022-12-05)
- Added Device module that simulates an ECHONET-Lite device node
- Added a controller example that posts a message to other nodes as uechopost

## v0.9.3 (2022-12-03)
- Improved Controller to bind multiple interfaces
- Improved Controller not to bind unavailable interfaces

## v0.9.2 (2022-11-27)
- Update Controller to add the standard node profile object as default
- Updated StandardDatabase to add experimentamanufacture codes

## v0.9.1 (2022-11-26)
- Updated StandardDatabase to store official registerd manufactures by the ECHONET CONSORTIUM
- Updated uechosearch to print the searched node with the manufacture name

## v0.9.0 (2022-11-26)
- Initial public release  
- Added Controller that represents an ECHONET-Lite controller node to communicate other ECHONET-Lite nodes
- Added uechosearch as an example of Controller
