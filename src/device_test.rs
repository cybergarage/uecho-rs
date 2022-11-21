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

#[cfg(test)]
mod tests {

    use crate::device::*;
    use crate::super_object::{OBJECT_OPERATING_STATUS_ON, SUPER_OBJECT_CODE};

    #[test]
    fn device() {
        let mut dev = Device::new(SUPER_OBJECT_CODE);
        dev.set_operating_status(true);
        assert_eq!(
            dev.operating_status().byte_data(),
            OBJECT_OPERATING_STATUS_ON
        );
        dev.set_installation_location(DEVICE_INSTALLATION_LOCATION_UNKNOWN);
        assert_eq!(
            dev.installation_location().byte_data(),
            DEVICE_INSTALLATION_LOCATION_UNKNOWN
        );
    }
}
