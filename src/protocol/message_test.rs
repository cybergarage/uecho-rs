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

    use crate::protocol::esv::*;
    use crate::protocol::message::*;
    use hex;

    #[test]
    fn new_message() {
        let mut msg = Message::new();
        assert!(msg.is_format1());
    }

    #[test]
    fn message_setter() {
        let mut msg = Message::new();

        let test_tid = 0x1234 as TID;
        msg.set_tid(test_tid);
        assert_eq!(msg.tid(), test_tid);

        let test_seoj = 0x2345 as u32;
        msg.set_seoj(test_seoj);
        assert_eq!(msg.seoj(), test_seoj);

        let test_deoj = 0x3456 as u32;
        msg.set_deoj(test_deoj);
        assert_eq!(msg.deoj(), test_deoj);

        let test_esv = Esv::WriteRequest;
        msg.set_esv(test_esv);
        assert_eq!(msg.esv(), test_esv);
    }

    #[test]
    fn message_parse() {
        fn message_parse(msg_bytes: &[u8]) {
            // Checks parsed result

            let mut msg = Message::new();

            assert!(msg.parse(msg_bytes), "{}", hex::encode_upper(msg_bytes));
            assert_eq!(msg.tid(), 0x0101);
            assert_eq!(msg.seoj(), 0x0A0B0C0);
            assert_eq!(msg.deoj(), 0x0D0E0F0);
            assert_eq!(msg.esv(), Esv::Notification);

            let opc = msg.opc();
            for i in 0..opc {
                let prop = msg.property(i);
                let prop_size = prop.size();
                assert_eq!(prop_size, (i + 1));
                let prop_data = prop.data();
                assert_eq!(prop_data.len(), (i + 1));
                for j in 0..prop_size {
                    let prop_val = (0x61 + i + j) as u8;
                    assert_eq!(prop_data[j], prop_val)
                }
            }

            // Checks exported message

            let msg_bytes = msg.bytes();
            let mut parsed_msg = Message::new();
            assert!(parsed_msg.parse(&msg_bytes));
            assert!(msg.equals(&parsed_msg));
        }

        let mut test_msg = [
            HEADER_EHD1_ECHONET,
            HEADER_EHD2_FORMAT1,
            0x01,
            0x01,
            0xA0,
            0xB0,
            0xC0,
            0xD0,
            0xE0,
            0xF0,
            Esv::to_u8(Esv::Notification),
            3,
            1,
            1,
            0x61, // a
            2,
            2,
            0x62, // b
            0x63, // c
            3,
            3,
            0x63, // d
            0x64, // e
            0x65, // f
        ];

        let mut prop_data_size = 0;
        for n in 0..3 {
            let opc_idx = FRAME_HEADER_SIZE + FORMAT1_HEADER_SIZE - 1;
            let test_msg_size = FRAME_HEADER_SIZE + FORMAT1_HEADER_SIZE + prop_data_size;
            test_msg[opc_idx] = n;
            message_parse(&test_msg[..test_msg_size]);
            prop_data_size += (1 + 1 + (n + 1)) as usize;
        }
    }

    #[test]
    fn message_parse_requests() {
        let mut test_msgs = Vec::new();
        test_msgs.push(hex::decode("1081000100F00100F0016201D600").ok().unwrap());

        for test_msg_bytes in test_msgs {
            let mut msg = Message::new();
            assert!(
                msg.parse(&test_msg_bytes),
                "{}",
                hex::encode_upper(&test_msg_bytes)
            );
        }
    }

    #[test]
    fn message_from_message() {
        let msg_bytes = hex::decode("108100010EF0010EF0017201D607020F2001029101")
            .ok()
            .unwrap();
        let _msg = Message::from_message(&msg_bytes);
    }
}
