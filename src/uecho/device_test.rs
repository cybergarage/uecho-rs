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
        assert_eq!(dev.operating_status().byte_data(), ObjectOperatingStatusOn);
        assert_eq!(dev.installation_location().byte_data(), DeviceInstallationLocationUnknown);
    }
}
