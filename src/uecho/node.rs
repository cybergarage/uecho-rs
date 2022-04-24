// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::object::*;

trait Node {
    fn objects(&mut self) -> &Objects;
    fn address(self) -> String;

    fn add_object(&mut self, obj: Object) -> bool {
        self.objects().lock().unwrap().push(obj);
        true
    }
}
