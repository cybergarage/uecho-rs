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

    use crate::database::*;
    use crate::node_profile::*;
    use crate::super_object::*;

    #[test]
    fn standard_database() {
        let db = StandardDatabase::shared();

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
