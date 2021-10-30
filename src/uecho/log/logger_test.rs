// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(test)]
mod tests {

    use log::*;

    use crate::uecho::log::logger;

    #[test]
    fn logger_test() {
        logger::init();
        error!("");
        warn!("");
        info!("");
        debug!("");
        trace!("");
    }
}
