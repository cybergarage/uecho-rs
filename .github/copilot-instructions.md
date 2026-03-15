# Copilot Instructions

## Project Overview

**uecho-rs** (published as crate `echonet`) is a Rust framework for the [ECHONET Lite](https://echonet.jp/english/) IoT protocol, used by home appliances in Japan. It supports both **controller** nodes (discover/control devices) and **device** nodes (implement IoT devices), plus an encoder/decoder and a standard device database built from the Machine Readable Appendix (MRA).

## Build, Test, and Lint

```bash
make build        # cargo fmt + cargo build
make test         # cargo fmt + cargo build + cargo test -- --test-threads=1
make format       # cargo fmt
make doc          # cargo doc --open
```

**Tests must run single-threaded** — always pass `--test-threads=1`:

```bash
# Run full test suite
cargo test -- --test-threads=1

# Run a single test by name
cargo test <test_name> -- --test-threads=1

# Run a specific integration test file
cargo test --test node_test -- --test-threads=1

# Run a unit test in a specific module
cargo test --lib transport::manager_test -- --test-threads=1
```

Run CLI tools:

```bash
cargo run --bin uechosearch -v
cargo run --bin uechopost -v
cargo run --example monolight -v
```

## Architecture

The crate is organized into three layers:

1. **Domain layer** (`src/`) — `Node`, `Object`, `Property`, `RemoteNode`, `Device`, `Controller`. A `Node` contains `Object`s, each with `Property` values. A `Device` wraps a `Node` for server-side use; a `Controller` wraps a `Node` for client-side discovery and control.

2. **Protocol layer** (`src/protocol/`) — `Message` (ECHONET packet), `ESV` (18 service code enum: `WriteRequest`, `ReadRequest`, `Notification`, etc.), and protocol-level `Property`.

3. **Transport layer** (`src/transport/`) — UDP multicast/unicast managers, socket abstractions, and the `Observer` notification system.

The `StandardDatabase` is a singleton (accessed via `StandardDatabase::shared()`) that holds MRA object/property definitions and manufacturer codes generated from `src/std/`.

Public API surface is defined in `src/lib.rs` via `pub use` re-exports.

**Platform features** (selected in `Cargo.toml`):
- `unix` (default) — uses `pnet`, `nix`, `net2`
- `win` — Windows support
- `esp` — ESP32 / no_std support

## Key Conventions

### Concurrency
Shared state uses `Arc<Mutex<T>>`. Constructors like `Node::new()` return `Arc<Mutex<Node>>`. Locks are taken with `.lock().unwrap()`. Inter-thread communication uses `mpsc` channels.

### Test placement
- Unit tests live in colocated `*_test.rs` files (e.g., `src/transport/manager_test.rs`), not inside the implementation file.
- Integration tests live in `tests/` and share utilities via `tests/test.rs`.

### Public traits
Two extension points:
- `RequestHandler` — implement to handle property read/write/notify requests on a `Device`:
  ```rust
  fn property_request_received(&mut self, deoj: &mut Object, esv: ESV, prop: &Property) -> bool
  ```
- `Observer` — implement to receive raw `Message` events from a node.

### Type aliases
| Alias | Underlying | Notes |
|---|---|---|
| `ObjectCode` | `u32` | 3-byte group/class/instance |
| `PropertyCode` | `u8` | 0x80–0xFF |
| `PropertyData` | `Vec<u8>` | Big-endian binary |
| `ManufactureCode` | `u16` | |

### ESV enum
`ESV` uses `strum_macros` for iteration. Use `ESV::from_u8()` to decode, `.is_request()` / `.is_response()` for classification.

### Byte encoding
Use `util::Bytes::from_u32()` / `Bytes::to_u32()` for big-endian property data encoding — do not hand-roll byte manipulation.

### Builder pattern
Setters return `&mut Self` for method chaining (e.g., `object.set_code(0x05FD).set_name("light")`).

### Commit style
Short, imperative commit messages. One logical change per commit. Run `make test` before pushing.
