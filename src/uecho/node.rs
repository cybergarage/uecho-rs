// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::net::IpAddr;

use crate::uecho::object::*;

pub trait Node {
    fn objects(&mut self) -> &Objects;
    fn addr(&self) -> IpAddr;

    fn add_object(&mut self, obj: Object) -> bool {
        self.objects().lock().unwrap().push(obj);
        true
    }

    fn get_object(&mut self, code: ObjectCode) -> Option<&Object> {
        for (_, obj) in self.objects().lock().unwrap().iter().enumerate() {
            if obj.code() == code {
                return Some(&*obj);
            }
        }
        None
    }
}
