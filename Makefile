###################################################################
#
# uecho-rs
#
# Copyright (C) The uecho-rs Authors 2019
#
# This is licensed under BSD-style license, see file COPYING.
#
###################################################################

all: test

format:
	cargo fmt

build: format
	cargo build

test: build
	cargo test
