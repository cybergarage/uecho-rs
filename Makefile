###################################################################
#
# Copyright (C) 2022 The uecho-rs Authors All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
###################################################################

# export RUSTFLAGS := "-A dead_code"

all: test

.PHONY: format search mono bench

format:
	cargo fmt

build: format
	cargo build

clean: 
	cargo clean

doc: format
	cargo doc --open

test: build
	cargo test -- --test-threads=1

search:
	cargo run --bin uechosearch -v

post:
	cargo run --bin uechopost -v

mono:
	cargo run --example monolight -v

bench:
	cargo run --bin uechobench -v -- -n 10000

watchtest:
	fswatch -o . -e ".*" -i "\\.rs$$" | xargs -n1 -I{} make test
