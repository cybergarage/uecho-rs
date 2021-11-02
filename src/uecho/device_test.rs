// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[test]

#[cfg(test)]
mod tests {

    use crate::uecho::device::*;

    #[test]
    fn device_new_test() {
        let dev = Device::new();
        assert_eq!(dev.operating_status().byte_data(), OBJECT_OPERATING_STATUS_ON);
        assert_eq!(dev.installation_location().byte_data(), DEVICE_INSTALLATION_LOCATION_UNKNOWN);
        assert_eq!(dev.standard_version().byte_data(), [0x01, 0x00, DEVICE_STANDARD_VERSION, 0x00]);
        assert_eq!(dev.operating_status().byte_data(), DEVICE_NO_FAULT_OCCURRED);
        }
}


