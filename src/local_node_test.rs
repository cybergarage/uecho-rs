// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::local_node::*;
    use crate::node::*;

    #[test]
    fn local_node_object_test() {
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
    fn local_node_test() {
        let mut node = LocalNode::new();
        assert!(node.start());
        assert!(node.stop());
    }
}