// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::protocol::esv::*;
    use crate::uecho::protocol::message::*;
    use crate::uecho::transport::unicast_udp_server::*;

    #[test]
    fn unicast_udp_server_test() {
        let mut server = UnicastUdpServer::new();
        assert!(server.start());
        let server_addr = server.local_addr();
        assert!(server_addr.is_ok());

        let mut msg = Message::new();
        msg.set_esv(ESV_READ_REQUEST);
        assert!(server.send_message(server_addr.unwrap(), &msg));

        assert!(server.stop());
    }
}
