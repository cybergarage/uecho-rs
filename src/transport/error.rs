// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::io::{Error, ErrorKind};

pub struct ScoketError {}
pub struct BindError {}

impl ScoketError {
    pub fn new(msg: &str) -> Error {
        Error::new(ErrorKind::Other, msg)
    }
}

impl BindError {
    pub fn new() -> Error {
        ScoketError::new("socket is not bound")
    }
}
