// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::manufacture::*;
use crate::object::*;
use once_cell::sync::Lazy;

static SHARED_STANDARD_DATABASE: Lazy<StandardDatabase> = Lazy::new(|| StandardDatabase::new());

pub fn get_shared_standard_database() -> &'static Lazy<StandardDatabase> {
    &SHARED_STANDARD_DATABASE
}

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
        for n in 0..self.objects.len() {
            if self.objects[n].code() == code {
                return Some(&self.objects[n]);
            }
        }
        None
    }
}
