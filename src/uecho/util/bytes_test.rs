// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::util::bytes::*;

    #[test]
    fn bytes_from_test() {
        let mut buf: [u8; 1] = [0; 1];
        for n in 0..=0xFF {
            Bytes::from_u32(n, &mut buf);
            assert_eq!(n, Bytes::to_u32(&buf));
        }

        let mut buf2: [u8; 2] = [0; 2];
        for n in (0..=0xFFFF).step_by(0xFFFF / 0xFF) {
            Bytes::from_u32(n, &mut buf2);
            assert_eq!(n, Bytes::to_u32(&buf2));
        }

        let mut buf: [u8; 3] = [0; 3];
        for n in (0..=0xFFFFFF).step_by(0xFFFFFF / 0xFF) {
            Bytes::from_u32(n, &mut buf);
            assert_eq!(n, Bytes::to_u32(&buf));
        }

        let mut buf: [u8; 4] = [0; 4];
        for n in (0..=0xFFFFFFFFu32).step_by(0xFFFFFFFF / 0xFF) {
            Bytes::from_u32(n, &mut buf);
            assert_eq!(n, Bytes::to_u32(&buf));
        }
    }
}
