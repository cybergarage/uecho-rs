// Copyright (C) 2022 The uecho-rs Authors All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod tests {

    use crate::object::*;
    use crate::protocol::Message;
    use crate::remote_node::*;

    #[test]
    fn remote_node() {
        let mut node = RemoteNode::new();
        for n in 1..10 {
            let mut obj = Object::new();
            assert!(obj.set_code(n));
            assert!(node.add_object(obj));
        }
    }

    // #[test]
    // fn remote_node_from_message() {
    //     let msg = Message::
    //     let mut node = RemoteNode::from_message("");
    //     for n in 1..10 {
    //         let mut obj = Object::new();
    //         assert!(obj.set_code(n));
    //         assert!(node.add_object(obj));
    //     }
    // }
}
