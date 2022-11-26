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

use crate::manufacture::*;
use crate::object::*;
use once_cell::sync::Lazy;

static SHARED_STANDARD_DATABASE: Lazy<StandardDatabase> = Lazy::new(|| StandardDatabase::new());

/// StandardDatabase represents a standard database for official device objects and manufactures defined by the ECHONET CONSORTIUM.
pub struct StandardDatabase {
    manufactures: Vec<Manufacture>,
    objects: Vec<Object>,
}

impl StandardDatabase {
    pub fn new() -> StandardDatabase {
        let mut db = StandardDatabase {
            manufactures: Vec::new(),
            objects: Vec::new(),
        };
        db.init_manufactures();
        db.init_objects();
        db
    }

    pub fn shared() -> &'static Lazy<StandardDatabase> {
        &SHARED_STANDARD_DATABASE
    }

    pub fn add_manufacture(&mut self, man: Manufacture) -> bool {
        self.manufactures.push(man);
        true
    }

    pub fn find_manufacture(&self, code: ManufactureCode) -> Option<&Manufacture> {
        for n in 0..self.manufactures.len() {
            if self.manufactures[n].code() == code {
                return Some(&self.manufactures[n]);
            }
        }
        None
    }

    pub fn add_object(&mut self, obj: Object) -> bool {
        self.objects.push(obj);
        true
    }

    pub fn find_object(&self, code: ObjectCode) -> Option<&Object> {
        let class_group_code = code & 0xFFFF00;
        for n in 0..self.objects.len() {
            if self.objects[n].code() == class_group_code {
                return Some(&self.objects[n]);
            }
        }
        None
    }
}
