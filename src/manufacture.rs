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

use std::sync::Arc;
use std::sync::Mutex;

pub type ManufactureCode = u32;

/// Manufacture represents a manufacturer name and code registerd by the ECHONET CONSORTIUM.
pub struct Manufacture {
    code: ManufactureCode,
    name: String,
}

pub type Manufactures = Arc<Mutex<Vec<Manufacture>>>;

impl Manufacture {
    pub fn new() -> Manufacture {
        Manufacture {
            code: 0,
            name: String::from(""),
        }
    }

    pub fn set_code(&mut self, code: ManufactureCode) {
        self.code = code;
    }

    pub fn code(&self) -> ManufactureCode {
        self.code
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
