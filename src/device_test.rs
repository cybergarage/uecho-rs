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
    use crate::super_object::*;
    use crate::util::Bytes;

    #[test]
    fn device_mandatory_properties() {
        let device_code = 0x029101;
        let mut dev = Device::new(device_code);

        let prop_data = dev.property(DEVICE_STANDARD_VERSION);
        assert!(prop_data.is_some());
        let prop_data = prop_data.unwrap();
        assert_eq!(prop_data.len(), 4);
        assert_eq!(prop_data[2], DEVICE_DEFAULT_VERSION_APPENDIX);

        let prop_data = dev.property(DEVICE_FAULT_STATUS);
        assert!(prop_data.is_some());
        let prop_data = prop_data.unwrap();
        assert_eq!(prop_data.len(), 1);
        assert_eq!(Bytes::to_u32(&prop_data) as u8, DEVICE_NO_FAULT_OCCURRED);

        let prop_data = dev.property(DEVICE_INSTALLATION_LOCATION);
        assert!(prop_data.is_some());
        let prop_data = prop_data.unwrap();
        assert_eq!(prop_data.len(), 1);
        assert_eq!(
            Bytes::to_u32(&prop_data) as u8,
            DEVICE_INSTALLATION_LOCATION_UNKNOWN
        );

        let prop_data = dev.property(DEVICE_MANUFACTURER_CODE);
        assert!(prop_data.is_some());
        let prop_data = prop_data.unwrap();
        assert_eq!(prop_data.len(), 3);
        assert_eq!(Bytes::to_u32(&prop_data), DEVICE_MANUFACTURER_EXPERIMENT);

        assert!(dev.set_property(DEVICE_OPERATING_STATUS, &[OBJECT_OPERATING_STATUS_ON]));
        let prop_data = dev.property(DEVICE_OPERATING_STATUS);
        assert!(prop_data.is_some());
        assert_eq!(prop_data.unwrap(), &[OBJECT_OPERATING_STATUS_ON]);

        assert!(dev.set_property(DEVICE_OPERATING_STATUS, &[OBJECT_OPERATING_STATUS_OFF]));
        let prop_data = dev.property(DEVICE_OPERATING_STATUS);
        assert!(prop_data.is_some());
        assert_eq!(prop_data.unwrap(), &[OBJECT_OPERATING_STATUS_OFF]);

        assert!(dev.start());
        let prop_data = dev.property(DEVICE_OPERATING_STATUS);
        assert!(prop_data.is_some());
        assert_eq!(prop_data.unwrap(), &[OBJECT_OPERATING_STATUS_ON]);

        assert!(dev.stop());
        let prop_data = dev.property(DEVICE_OPERATING_STATUS);
        assert!(prop_data.is_some());
        assert_eq!(prop_data.unwrap(), &[OBJECT_OPERATING_STATUS_OFF]);
    }
}
