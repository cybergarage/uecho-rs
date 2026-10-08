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

    use crate::object::*;
    use crate::property::*;

    #[test]
    fn object_code() {
        let mut obj = Object::new();
        obj.set_code(0x0EF001);
        assert_eq!(obj.code(), 0x0EF001);
        assert_eq!(obj.class_group_code(), 0x0E);
        assert_eq!(obj.class_code(), 0xF0);
        assert_eq!(obj.instance_code(), 0x01);
    }

    #[test]
    fn object_property() {
        let mut obj = Object::new();

        for n in 1..10 {
            let mut prop = Property::new();
            prop.set_code(n as PropertyCode);
            assert!(obj.add_property(prop));
        }

        for n in 1..10 {
            let prop = obj.find_property_mut(n as PropertyCode);
            assert!(prop.is_some());
        }
    }

    // Decode independently of the encoder, using Annex 1's byte/bit table.
    fn decode_property_map(data: &[u8]) -> Vec<PropertyCode> {
        let count = data[0] as usize;
        let mut codes = if count < 16 {
            assert_eq!(data.len(), count + 1);
            data[1..].to_vec()
        } else {
            assert_eq!(data.len(), 17);
            let mut codes = Vec::new();
            for (row, byte) in data[1..].iter().enumerate() {
                for (bit, base) in [0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0]
                    .iter()
                    .enumerate()
                {
                    if byte & (1 << bit) != 0 {
                        codes.push(base + row as u8);
                    }
                }
            }
            codes
        };
        assert_eq!(codes.len(), count);
        codes.sort_unstable();
        codes
    }

    #[test]
    fn property_map_encoding_fixtures() {
        use crate::super_object::{
            OBJECT_ANNO_PROPERTY_MAP, OBJECT_GET_PROPERTY_MAP, OBJECT_SET_PROPERTY_MAP,
        };

        let fixtures: Vec<(&str, Vec<PropertyCode>, Vec<u8>)> = vec![
            ("empty", vec![], vec![0]),
            ("one_upper", vec![0xFF], vec![1, 0xFF]),
            (
                "format1_15",
                (0xC0..=0xCE).collect(),
                vec![
                    15, 0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xCB,
                    0xCC, 0xCD, 0xCE,
                ],
            ),
            (
                "format2_16",
                (0xC0..=0xCF).collect(),
                [vec![16], vec![0x10; 16]].concat(),
            ),
            (
                "format2_17_mixed",
                (0x80..=0x8F).chain([0xFF]).collect(),
                vec![17, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0x81],
            ),
            (
                "all_upper",
                (0xC0..=0xFF).collect(),
                [vec![64], vec![0xF0; 16]].concat(),
            ),
            (
                "all_epcs",
                (0x80..=0xFF).collect(),
                [vec![128], vec![0xFF; 16]].concat(),
            ),
        ];
        let map_codes = [
            OBJECT_GET_PROPERTY_MAP,
            OBJECT_SET_PROPERTY_MAP,
            OBJECT_ANNO_PROPERTY_MAP,
        ];
        for (name, codes, expected) in fixtures {
            let mut obj = Object::new();
            for code in &codes {
                let mut prop = Property::new();
                prop.set_code(*code)
                    .set_read_attribute(PropertyRule::Optional)
                    .set_write_attribute(PropertyRule::Optional)
                    .set_anno_attribute(PropertyRule::Optional);
                assert!(obj.add_property(prop));
            }
            // Add map storage only when absent, without including extra EPCs.
            for map_code in map_codes {
                if !obj.has_property(map_code) {
                    let mut prop = Property::new();
                    prop.set_code(map_code);
                    assert!(obj.add_property(prop));
                }
            }
            for map_code in map_codes {
                let data = obj.property_data_as_bytes(map_code).unwrap();
                if codes.len() < 16 {
                    assert_eq!(data[0], expected[0], "{name}: map {map_code:02X}");
                } else {
                    assert_eq!(data, expected, "{name}: map {map_code:02X}");
                }
                assert_eq!(
                    decode_property_map(data),
                    codes,
                    "{name}: map {map_code:02X}"
                );
            }
        }
    }
}
