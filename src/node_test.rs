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

    use crate::node::*;
    use crate::node_profile::*;
    use crate::super_object::*;

    #[test]
    fn node() {
        let node = Node::new();
        let mut node = node.lock().unwrap();

        let obj = node.find_object(NODE_PROFILE_OBJECT_CODE);
        assert!(obj.is_some());
        let obj = obj.unwrap();
        let man_code = obj.property_data_as_int(OBJECT_MANUFACTURER_CODE);
        assert!(man_code.is_some());
        assert_eq!(man_code.unwrap(), OBJECT_MANUFACTURER_EXPERIMENT);
        let obj_id = obj.property_data_as_bytes(NODE_PROFILE_CLASS_IDENTIFICATION_NUMBER);
        assert!(obj_id.is_some());
        assert!(0 < obj_id.unwrap().len());

        assert!(node.start());
        assert!(node.stop());
    }
}
