![logo](img/logo.png)

# Overview of uEcho Device

The `uecho-rs` supports your original standard devices of [ECHONET Lite][enet] specification easily. This document explains to create your original  [ECHONET Lite][enet] device step by step.

## Creating Devices

### 1. Creating Device Node

To create your original device, use `echonet::Device` with a device object code as the following at first.


```
pub struct MyDevice {
    pub device: Device,
}

impl MyDevice {
    pub fn new() -> Arc<Mutex<MonoLight>> {
        let m = Arc::new(Mutex::new(MonoLight {
            device: Device::new(0x029101),
        }));
        m
    }
}
```

The new node has the specified device object and the node profile class object. The node profile object is updated automatically when new devices are added into the node or the any properties in the node are changed.

### 2. Handling Request Messages 

To implement the device object, you have only to handle request messages from other nodes because the `uecho-rs` handles other standard read and notification requests automatically.  The `Device::set_request_handler()` can set the following permission handler to a device object to handle request messages from other nodes. The handler should return true if the request message is allowed, otherwise false as the following.

```
impl MyDevice {
    pub fn new() -> Arc<Mutex<MonoLight>> {
        let m = Arc::new(Mutex::new(MonoLight {
            device: Device::new(0x029101),
        }));
        m.lock().unwrap().device.set_request_handler(m.clone());
        m
    }
}

impl RequestHandler for MyDevice {
    fn property_request_received(&mut self, deoj: ObjectCode, esv: Esv, prop: &Property) -> bool {
        // Ignore all messages to other objects in the same node.
        if deoj != self.device.code() {
            return false;
        }
        match esv {
            Esv::WriteRequest | Esv::WriteReadRequest => {
                let prop_code = prop.code();
                let prop_bytes = prop.data();
                match prop_code {
                    ....
                }
            }
            _ => {}
        }
        true
    }
}
```

 In addition, the developer does not need to update the target property data by the request property data because the `uecho-rs` updates the target property by the request property data automatically when the handler returns true.

### 4. Starting Node

Finally, start the created node which has your device objects to use `Device::start()` as the following:

```
let my_dev = MonoLight::new();
my_dev.lock().unwrap().device.start();
```

## Next Steps

Let's check the following documentation to know the device functions of the `uecho-rs` in more detail.

- [Inside of uEcho Device](https://github.com/cybergarage/uecho-rs/blob/master/doc/device_inside.md)
- [Usage examples](https://github.com/cybergarage/uecho-rs/tree/master/examples)

## References

- \[1\] [Detailed Requirements for ECHONET Device objects][enet-spec]

[enet]:http://echonet.jp/english/
[enet-spec]:http://www.echonet.gr.jp/english/spec/index.htm
