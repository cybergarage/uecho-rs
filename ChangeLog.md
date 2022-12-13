# Changelog

## v1.3.x (2022-xx-xx)
- Enable IPv6 interfaces

## v1.2.x (2022-xx-xx)
- Improve controller search functions
- Improve uechosearch example to output more detailed properties of searched devices using the standard database

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
