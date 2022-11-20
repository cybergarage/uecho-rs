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

    use crate::protocol::property::*;

    #[test]
    fn property_parse_test() {
        let test_prop_bytes = &[
            10, 1, 0x61, // a
        ];

        let mut prop = Property::new();

        assert!(prop.parse(test_prop_bytes));
        assert_eq!(prop.code(), 10u8);
        assert_eq!(prop.size(), 1);
        assert_eq!(prop.data()[0], 0x61u8)
    }
}
