// Copyright (C) 2022 Satoshi Konno All rights reserved.
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

use std::sync::Once;

use chrono::{Local, SecondsFormat};
use log::{LevelFilter, Metadata, Record};

static INIT: Once = Once::new();

struct DefaultLogger {
    level: LevelFilter,
}

impl DefaultLogger {
    pub fn set_level(&self, _: LevelFilter) {}
}

impl log::Log for DefaultLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let ts = Local::now();
            println!(
                "{} {} {}",
                ts.to_rfc3339_opts(SecondsFormat::Secs, true),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: DefaultLogger = DefaultLogger {
    level: LevelFilter::Trace,
};

pub fn init() {
    INIT.call_once(|| {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Trace))
            .expect("Couldn't initialize logger");
    });
}

pub fn set_level(level: LevelFilter) {
    LOGGER.set_level(level);
}

pub fn enable_debug() {
    set_level(LevelFilter::Debug);
}

pub fn enable_trace() {
    set_level(LevelFilter::Trace);
}
