![logo](img/logo.png)

# Overview of uEcho Controller

The controller is a special node of [ECHONETLite][enet] to control other nodes, it can find the nodes in the local area network and send any messages into the found devices.

## Using Controller

### 1. Starting Controller

To start a controller, create a controller instanse of `echonet::Controller` and start the controller using `Controller::start()` as the following:

```
use echonet::Controller;

let mut ctrl = Controller::new();
ctrl.start();
```

### 2. Searching Nodes

Next, use `Controller::search()` to search other nodes in the local area network as the following:

```
let mut ctrl = Controller::new()
....
ctrl.search()
```

### 3. Getting Nodes and Objects

After the searching, the controller has the found nodes in the `Controller::nodes` property. The [ECHONETLite](http://www.echonet.gr.jp/english/index.htm) node might have multiple objects such as the device or profile objects, and the found node has the objects in the `Object.objects` property. The following example shows all objects in the found nodes.

```
let mut ctrl = Controller::new()
....
for (i, node) in ctrl.nodes().iter().enumerate() {
    println!("[{}] {}", i, node.addr());
    for (j, obj) in node.objects().iter().enumerate() {
        println!("[{}] {:06X}", j, obj.code());
        for obj_prop in obj.properties() {
            ....
        }
    }
}
```

### 4. Creating Request Message

To control the found objects, create the request message using `echonet::Message` as the following.

```
use echonet::protocol::{Esv, Message, Property};
....
let mut msg = Message::new();
msg.set_esv(Esv::ReadRequest);
msg.set_deoj(obj.code());
let mut prop = Property::new();
prop.set_code(obj_prop.code());
msg.add_property(prop);
```

### 5. Sending Messages

To send the created request message, use `Controller::send_message()` as the following:

```
let mut ctrl = Controller::new();
....
let node = ctrl.nodes[0];
....
let mut msg = Message::new();
....
ctrl.send_message(&node, &mut msg);
```

Basically, all messages of [ECHONETLite](http://www.echonet.gr.jp/english/index.htm) is async. To handle the async response message, use `Controller::post_message()` instead of `Controller::send_message()` as the following:

```
let rx = ctrl.post_message(&node, &mut msg);
match rx.recv_timeout(Duration::from_secs(1)) {
    Ok(msg) => {
        ....
    }
    Err(_e) => {
        ....
    }
};
```

## Next Steps

Let's check the following documentation to know the controller functions of uEcho in more detail.

- [Usage examples](https://github.com/cybergarage/uecho-rs/tree/master/examples)

[enet]:http://echonet.jp/english/
