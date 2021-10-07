// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[test]

#[cfg(test)]
mod tests {

    use crate::uecho::property::*;

    #[test]
    fn property_set_test() {
        let mut prop = Property::new();
        for n in 1..10 {
            let data = vec![0; n];
            prop.set_data(data);
            assert_eq!(prop.size(), n);
        }
    }
}
