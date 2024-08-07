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

    use std::{thread, time};
    use crate::controller::*;
    use crate::log::Logger;

    #[test]
    fn controller() {
        Logger::init();
        {
            let slp = time::Duration::from_secs(1);
            let mut ctrl = Controller::new();
            assert!(ctrl.start());
            thread::sleep(slp);
            assert!(ctrl.search());
            thread::sleep(slp);
            assert!(ctrl.stop());
            thread::sleep(slp);
        }
    }
}
