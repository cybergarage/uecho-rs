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

    use crate::local_node::*;
    use crate::node::*;

    #[test]
    fn local_node_object() {
        let mut node = LocalNode::new();
        for n in 1..10 {
            let obj = Object::new();
            assert!(obj.set_code(n));
            assert!(obj.add_object(obj));
        }

        for n in 1..10 {
        }
    }

    #[test]
    fn local_node() {
        let mut node = LocalNode::new();
        assert!(node.start());
        assert!(node.stop());
    }
}
