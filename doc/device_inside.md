![logo](img/logo.png)

# Inside of uEcho Device

A [ECHONET Lite][enet] node includes the objects, and the objects can be classified into two types: profile objects and device objects. The profile object is standard profile information such as the operation status, manufacturer information, and device object list. The device object is a logical model of the information or of control items that can be remotely controlled. 

![Device Objects](img/device_objects.png)

## Node Profile Object

The node profile object is a standard profile object, [ECHONET Lite][enet] node must have the following mandatory profile class object which has all children objects of the node [\[1\]][enet-spec].

- Class group code: 0x0E
- Class code: 0xF0
- Instance code: 0x01 (general node)

The `uecho-rs` updates the node profile class objects automatically when the children objects in the node are changed, and so the developer doesn't need to update the node profile object yourself.

## Device Message Handler and Observer

The `uecho-rs` handles all request messages from other nodes automatically, the developer need only control request message permissions from other nodes and controllers to the target object properties using the object property handlers of the `uecho-rs`. However, the developer can set some request message observers into the node and objects to listen the raw messages of [ECHONET Lite][enet] too. The following figure shows the message handling sequence of `uecho-rs`.

![Node Observers](img/node_msg_handler.png)

The request message observers of the node and object can listen all request messages, but the object property handlers receives only valid request messages.

### Property Message Handler

The `RequestHandler` can set the following permission handler to an object property to handle valid request messages from other nodes. 

```
pub trait RequestHandler {
    fn property_request_received(&mut self, deoj: ObjectCode, esv: Esv, prop: &Property) -> bool;
}
```

The developer handles the request messages from other nodes. The developer should return a true if the request message is valid, otherwise false. In addition, the developer does not need to update the target property data by the request property data because the `uecho-rs` updates the target property by the request property data automatically when the handler returns true. The following example shows to check a write request message and set the valid property data to the target property.

```
impl RequestHandler for MyDevice {
    fn property_request_received(&mut self, deoj: ObjectCode, esv: Esv, prop: &Property) -> bool {
        // Ignore all messages to other objects in the same node.
        if deoj != self.device.code() {
            return false;
        }
        match prop_code {
            0x80 /* Operating status */ => {
                let prop_u32 = Bytes::to_u32(prop_bytes);
                match prop_u32 {
                    0x30 /* On */=> {
                        return true;
                    }
                    0x31 /* Off */=> {
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            _ => {
                return false;
            }
        }
    }
}
```

## References

- \[1\] [Part II ECHONET Lite Communication Middleware Specification][enet-spec]
- \[2\] [Detailed Requirements for ECHONET Device objects][enet-spec]

[enet]:http://echonet.jp/english/
[enet-spec]:http://www.echonet.gr.jp/english/spec/index.htm
