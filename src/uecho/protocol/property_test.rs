// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::protocol::property::*;

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
