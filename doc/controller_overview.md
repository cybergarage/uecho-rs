![logo](img/logo.png)

# Overview of uEcho Controller

The controller is a special node of [ECHONETLite][enet] to control other nodes, it can find the nodes in the local area network and send any messages into the found devices.

## Using Controller

### 1. Starting Controller

To start a controller, create a controller instanse of `uecho::Controller` and start the controller using `Controller::start()` as the following:

```
from uecho import Controller

ctrl = Controller()
ctrl.start()
```

### 2. Searching Nodes

Next, use `Controller::search()` to search other nodes in the local area network as the following:

```
ctrl = Controller()
....
ctrl.search()
```

### 3. Getting Nodes and Objects

After the searching, the controller has the found nodes in the `Controller::nodes` property. The [ECHONETLite](http://www.echonet.gr.jp/english/index.htm) node might have multiple objects such as the device or profile objects, and the found node has the objects in the `Object.objects` property. The following example shows all objects in the found nodes.

```
ctrl = Controller()
....
for i, node in enumerate(ctrl.nodes):
    print("[%d] %s" % (i, node.ip));
    for j, obj in enumerate(node.objects):
        print('[%d] %06X ' % (j, obj.code));
```

### 4. Creating Request Message

To control the found objects, create the request message using `uecho::Message` as the following.

```
from uecho import Message, Property, ESV

msg = Message()
msg.DEOJ = 0xXXXXXX
msg.ESV = ESV.WRITE_REQUEST
prop = Property()
prop.code = 0xXX
prop.data = ....
msg.add_property(prop)
```

### 5. Sending Messages

To send the created request message, use `Controller::send_message()` as the following:

```
ctrl = Controller()
....
node = ctrl.nodes[0]
....
msg = Message()
....
ctrl.send_message(msg, node);
```

Basically, all messages of [ECHONETLite](http://www.echonet.gr.jp/english/index.htm) is async. To handle the async response message, use `Controller::post_message()` instead of `Controller::send_message()` as the following:

```
res_msg = ctrl.post_message(msg, node);
if res_msg is not None:
    print("%02X" % res_msg.ESV)
    for prop in res_msg.properties:
        print("%02X %s " % ((prop.code), prop.data.hex().upper()))
```

## Next Steps

Let's check the following documentation to know the controller functions of uEcho in more detail.

- [Examples](./examples.md)

[enet]:http://echonet.jp/english/
