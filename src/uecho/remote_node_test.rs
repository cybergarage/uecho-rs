// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::node::*;
    use crate::uecho::remote_node::*;

    #[test]
    fn remote_node_test() {
        let mut node = RemoteNode::new();
        for n in 1..10 {
            let obj = Object::new();
            assert!(obj.set_code(n));
            assert!(obj.add_object(obj));
        }

        for n in 1..10 {
        }
    }
}
