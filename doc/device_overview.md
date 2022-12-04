![logo](img/logo.png)

# Overview of uEcho Device

## Making Devices

The `uecho-py` supports your original standard devices of [ECHONET Lite][enet] specification easily. This document explains to create your original  [ECHONET Lite][enet] device step by step.

## Creating Devices

### 1. Creating Node

To create your original device, use `uecho::LocalNode` as the following at first.


```
from uecho import LocalNode

node = LocalNode()
```

The new node has only a node profile class object, and it has no device object. The node profile object is updated automatically when new devices are added into the node or the any properties in the node are changed.

### 2. Creating Device Object

To add your device objects into the created node, create a new device object using `uecho::Device` or `uecho::std::StandardDevice`, and add the created device object into the node as the following.

```
from uecho import LocalNode
from uecho.std import StandardDevice

node = LocalNode()
dev = StandardDevice(0x029101) # Mono functional lighting class
node.add_object(dev)
```

The `uecho::Device` creates a null device object with no properties, so you must add your own properties. In contrast, The `uecho::std::StandardDevice` creates a standard device object with the specified object code and adds the standard properties of the ECHONET device object specification [\[1\]][enet-spec] into the device object automatically.

### 3. Handling Request Messages 

To implement the device object, you have only to handle request messages from other nodes because the `uecho-py` handles other standard read and notification requests automatically.  The `Object::set_request_handler()` can set the following permission handler to a device object to handle request messages from other nodes. 

```
class ObjectRequestHandler(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def property_read_requested(self, prop: Property) -> bool:
    @abc.abstractmethod
    def property_write_requested(self, prop: Property, data: bytes) -> bool:
```

The handler should return true if the request message is allowed, otherwise false as the following.

```
from uecho import Property
from uecho.std import StandardDevice

class MyDevice(StandardDevice, ObjectRequestHandler):

    def __init__(self):
        super().__init__(0x029101)
        self.set_request_handler(self)

    def property_read_requested(self, prop: Property) -> bool:
        if prop.code != 0x80:
            return False
        return True

    def property_write_requested(self, prop: Property, data: bytes) -> bool:
        if prop.code != 0x80:
            return False
        if len(prop.data) != 1:
            return False
        if (data[0] != 0x30) and (data[0] != 0x31):
            return False
        return True
```

 In addition, the developer does not need to update the target property data by the request property data because the `uecho-py` updates the target property by the request property data automatically when the handler returns true.

### 4. Starting Node

Finally, start the created node which has your device objects to use `Node::start()` as the following:

```
from uecho import LocalNode

node = LocalNode()
....
node.start()
```

## Next Steps

Let's check the following documentation to know the device functions of the `uecho-py` in more detail.

- [Inside of uEcho Device](./device_inside.md)
- [uEcho Examples](./examples.md)

## References

- \[1\] [Detailed Requirements for ECHONET Device objects][enet-spec]

[enet]:http://echonet.jp/english/
[enet-spec]:http://www.echonet.gr.jp/english/spec/index.htm
