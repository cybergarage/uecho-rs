# Repository Guidelines

## Project Structure & Module Organization
- `src/` is the core library. Main protocol/runtime modules include `protocol/`, `transport/`, logging in `log/`, and utility helpers in `util/`.
- `tests/` contains integration tests (for example, `tests/node_test.rs`, `tests/test.rs`).
- `bin/` contains runnable tools: `uechosearch`, `uechopost`, and `uechobench`.
- `examples/` contains sample apps such as `examples/monolight/monolight.rs`.
- `src/std/` stores ECHONET standard assets (MRA JSON, manufacturer data) and helper scripts/Makefile for data handling.
- `doc/` contains architecture and usage docs.

## Build, Test, and Development Commands
- `make build`: formats then builds (`cargo fmt` + `cargo build`).
- `make test`: full local check used by this repo (`make build` then `cargo test -- --test-threads=1`).
- `cargo test -- --test-threads=1`: run tests directly with serialized test threads.
- `make doc`: generate docs (`cargo doc --open`).
- `cargo run --bin uechosearch -v` (or `uechopost`, `uechobench`): run CLI tools.
- `cargo run --example monolight -v`: run the sample device/controller app.

## Coding Style & Naming Conventions
- Use standard Rust formatting with `cargo fmt` (4-space indentation; no custom rustfmt config is checked in).
- Follow existing file/module naming style: snake_case modules and files (for example, `remote_node.rs`).
- Keep public APIs and error paths explicit; colocate related tests as `*_test.rs` in `src/` when unit-style coverage is preferred.

## Testing Guidelines
- Primary framework is Rust’s built-in test harness (`#[test]`, `cargo test`).
- Place cross-module behavior tests in `tests/`; keep focused module tests near implementation (`src/**/*_test.rs`).
- Match existing naming patterns: `*_test.rs` files and descriptive test function names.
- Ensure tests pass with single-thread execution (`--test-threads=1`), which CI and Makefile both use.

## Commit & Pull Request Guidelines
- Commit messages are short, imperative, and often prefixed (`style:`, `chore:`) when useful.
- Keep commits scoped (one logical change per commit) and run `make test` before pushing.
- PRs should include: purpose, affected modules, test evidence (command + result), and linked issue(s) when applicable.
- If protocol behavior or CLI output changes, include a brief example or output snippet.
