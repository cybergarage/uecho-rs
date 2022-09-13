// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[test]

#[cfg(test)]
mod tests {

    use crate::object::*;
    use crate::property::*;

    #[test]
    fn object_code_test() {
        let obj = Object::new();
        obj.set_code(0x0EF001);
        equals!(obj.code(), 0x0EF001);
        equals!(obj.group_code(), 0x0E);
        equals!(obj.class_code(),0xF0);
        equals!(obj.instance_code(),0x01 );

    #[test]
    fn object_property_test() {
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
