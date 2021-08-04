// Copyright (C) 2019 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::sync::Once;

use chrono::{Local, SecondsFormat};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

static INIT: Once = Once::new();

struct DefaultLogger;

impl log::Log for DefaultLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
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

static LOGGER: DefaultLogger = DefaultLogger;

pub fn init() {
    INIT.call_once(|| {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Info))
            .expect("Couldn't initialize logger");
    });
}
