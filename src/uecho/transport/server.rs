// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub trait Server {
    //fn address(self) -> String;
    //fn port(self) -> u16;
    fn start(&self) -> bool;
    fn stop(&self) -> bool;
}
