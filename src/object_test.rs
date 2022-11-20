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

#[test]

#[cfg(test)]
mod tests {

    use crate::object::*;
    use crate::property::*;

    #[test]
    fn object_code() {
        let obj = Object::new();
        obj.set_code(0x0EF001);
        equals!(obj.code(), 0x0EF001);
        equals!(obj.group_code(), 0x0E);
        equals!(obj.class_code(),0xF0);
        equals!(obj.instance_code(),0x01 );

    #[test]
    fn object_property() {
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
