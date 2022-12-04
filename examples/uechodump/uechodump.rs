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

use std::env;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use echonet::log::Logger;
use echonet::protocol::{Message, Observer};
use echonet::Controller;

/// MyController listens Echonet-Lite multicast protocol messages.
pub struct MyController {
    ctrl: Controller,
}

impl MyController {
    pub fn new() -> Arc<Mutex<MyController>> {
        let m = Arc::new(Mutex::new(MyController {
            ctrl: Controller::new(),
        }));
        m.lock().unwrap().ctrl.add_observer(m.clone());
        m
    }

    fn start(&mut self) -> bool {
        self.ctrl.start()
    }

    fn stop(&mut self) -> bool {
        self.ctrl.stop()
    }
}

impl Observer for MyController {
    fn message_received(&mut self, msg: &Message) {
        println!("{} : {}", msg.from().ip(), msg);
    }
}

fn main() -> Result<(), Error> {
    Logger::init();
    for arg in env::args() {
        print!("{}", arg);
        match arg.as_str() {
            "-v" => {
                Logger::init();
            }
            &_ => {}
        }
    }

    let ml = MyController::new();
    let mut ml = ml.lock().unwrap();
    ml.start();

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        thread::sleep(time::Duration::from_secs(1));
    }

    ml.stop();

    Ok(())
}
