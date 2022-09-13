// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::database_manufacturers::*;
use crate::database_mra_objects::*;
use crate::manufacture::*;
use crate::object::*;

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

    pub fn get_manufacture(&self, code: ManufactureCode) -> Option<&Manufacture> {
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

    pub fn get_object(&self, code: ObjectCode) -> Option<&Object> {
        for n in 0..self.objects.len() {
            if self.objects[n].code() == code {
                return Some(&self.objects[n]);
            }
        }
        None
    }
}
