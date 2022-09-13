// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::sync::Arc;
use std::sync::Mutex;

pub type ManufactureCode = u32;

pub struct Manufacture {
    code: ManufactureCode,
    name: String,
}

pub type Manufactures = Arc<Mutex<Vec<Manufacture>>>;

pub fn manufactures_new() -> Manufactures {
    Arc::new(Mutex::new(Vec::new()))
}

impl Manufacture {
    pub fn new() -> Manufacture {
        Manufacture {
            code: 0,
            name: String::from(""),
        }
    }

    pub fn set_code(&mut self, code: ManufactureCode) {
        self.code = code
    }

    pub fn code(&self) -> ManufactureCode {
        self.code
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
