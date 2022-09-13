// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::protocol::esv::*;
    use crate::protocol::message::*;

    #[test]
    fn new_message_test() {
        let mut msg = Message::new();
        assert!(msg.is_format1());
    }

    #[test]
    fn message_setter_test() {
        let mut msg = Message::new();

        let test_tid = 0x1234 as u32;
        msg.set_tid(test_tid);
        assert_eq!(msg.tid(), test_tid);

        let test_seoj = 0x2345 as u32;
        msg.set_source_object_code(test_seoj);
        assert_eq!(msg.source_object_code(), test_seoj);

        let test_deoj = 0x3456 as u32;
        msg.set_destination_object_code(test_deoj);
        assert_eq!(msg.destination_object_code(), test_deoj);

        let test_esv = Esv::WriteRequest;
        msg.set_esv(test_esv);
        assert_eq!(msg.esv(), test_esv);
    }

    #[test]
    fn message_parse_test() {
        fn message_parse(msg_bytes: &[u8]) {
            // Checks parsed result

            let mut msg = Message::new();

            assert!(msg.parse(msg_bytes), "{}", hex::encode_upper(msg_bytes));
            assert_eq!(msg.tid(), 0x0101);
            assert_eq!(msg.source_object_code(), 0x0A0B0C0);
            assert_eq!(msg.destination_object_code(), 0x0D0E0F0);
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
}