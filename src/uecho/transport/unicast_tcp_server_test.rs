// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use crate::uecho::transport::unicast_tcp_server::*;

    #[test]
    fn unicast_tcp_server_test() {
        let mut server = UnicastTcpServer::new();
        assert!(server.start());
        assert!(server.stop());
    }
}
