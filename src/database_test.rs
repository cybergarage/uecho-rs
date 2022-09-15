// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[test]

#[cfg(test)]
mod tests {

    use crate::database::*;
    use crate::super_object:*;
    use crate::node_profile:*;

    #[test]
    fn standard_database_test() {
        let db = get_shared_standard_database()

        // Super Object
        let obj = db.get_object(SUPER_OBJECT_CODE);
        assert(obj.is_some());
        let prop = obj.property(OBJECT_OPERATING_STATUS);
        assert(prop.is_some());

        // Node Profile
        let obj = db.get_object(NODE_PROFILE_OBJECT_CODE);
        assert(obj.is_some());
        let prop = obj.property(NODE_PROFILE_CLASS_NUMBER_OF_SELF_NODE_INSTANCES);
        assert(prop.is_some());

        // Mono functional lighting (0x0291)
        let obj = db.get_object(0x029100);
        assert(obj.is_some());
        let prop = obj.property(0xB0); // Light level Setting
        assert(prop.is_some());
    }
}
