// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[test]

#[cfg(test)]
mod tests {

    use crate::uecho::object::*;
    use crate::uecho::property::*;

    #[test]
    fn object_new_test() {
        let obj = Object::new();

        for n in 1..10 {
            let prop = Property::new();
            prop.set_code(n as PropertyCpde);
            assert!(obj.add_property(prop));
        }

        for n in 1..10 {
            let prop = obj.property(n as PropertyCpde);
            assert!(prop);
        }
    }
}
