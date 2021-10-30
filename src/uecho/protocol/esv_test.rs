// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::transport::esv::*;

    #[test]
    fn esv_test() {
        for esv in Esv::iter() {
            let u8esv = Esv::to_u8(esv);
            assert_eq!(esv, Esv::from_u8(u8esv));
        }
    }
}
