// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[test]

#[cfg(test)]
mod tests {

    use crate::property::*;

    #[test]
    fn property_set_data_test() {
        let mut prop = Property::new();
        for n in 1..10 {
            let data = vec![0; n];
            assert!(prop.set_data(data));
            assert_eq!(prop.size(), n);
        }
    }

    #[test]
    fn property_add_data_test() {
        let mut prop = Property::new();
        let total_data_size = 0;
        for n in 1..10 {
            let data = vec![0; n];
            prop.add_data(data);
            total_data_size += n;
            assert_eq!(prop.size(), total_data_size);
        }
    }
}
