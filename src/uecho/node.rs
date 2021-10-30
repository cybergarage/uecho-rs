// Copyright (C) 2021 Satoshi Konno. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::uecho::object::*;

trait Node {
    fn objects(self) -> Vec<Object>;
    fn address(self) -> String;
}
