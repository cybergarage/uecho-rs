// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::sync::Arc;
use std::sync::Mutex;

use crate::transport::notify_manager::DefaultNotifytManager;

pub type Notifier = Arc<Mutex<DefaultNotifytManager>>;

pub fn notifier_new() -> Notifier {
    Arc::new(Mutex::new(DefaultNotifytManager::new()))
}
