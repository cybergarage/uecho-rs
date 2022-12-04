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

    use crate::message::{
        NodeProfileMessage, ResponseErrorMessage, ResponseMessage, SearchMessage,
    };
    use crate::protocol::{Esv, Message};
    use hex;

    #[test]
    fn search_message() {
        let msg = SearchMessage::new();
        assert_eq!(
            msg.bytes(),
            hex::decode("108100000EF0010EF0016201D600").ok().unwrap()
        )
    }

    #[test]
    fn node_profile_message() {
        let msg =
            Message::from_bytes(&hex::decode("108100010EF0010EF0017201D6040105FF01").unwrap());
        let mut profile_msg = NodeProfileMessage::from_message(&msg);
        assert!(profile_msg.parse());
        assert_eq!(profile_msg.object_codes().len(), 1);
    }

    #[test]
    fn response_message() {
        let req_msg =
            Message::from_bytes(&hex::decode("108100330EF0010EF0016201D600").ok().unwrap());
        let res_msg = ResponseMessage::from(&req_msg);
        assert_eq!(res_msg.esv(), Esv::ReadResponse);
        assert_eq!(res_msg.tid(), 0x0033);
    }

    #[test]
    fn error_response_message() {
        let req_msg =
            Message::from_bytes(&hex::decode("108100330EF0010EF0016201D600").ok().unwrap());
        let res_msg = ResponseErrorMessage::from(&req_msg);
        assert_eq!(res_msg.esv(), Esv::ReadRequestError);
        assert_eq!(res_msg.tid(), 0x0033);
    }
}
