# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`uecho-rs` is a Rust framework for ECHONET-Lite IoT applications (Japanese standard for home electronics). Published as the `echonet` crate. It supports controllers (find/control devices), devices (implement device behavior), and cross-platform targets including Unix, Windows, and ESP32.

## Commands

```bash
# Build
make build          # cargo fmt + cargo build

# Test (must be single-threaded due to network state)
make test           # cargo build + cargo test --test-threads=1
cargo test --test-threads=1                    # all tests
cargo test <test_name> -- --test-threads=1     # single test

# Format
make format         # cargo fmt

# Docs
make doc            # cargo doc --open

# Run tools
make search         # cargo run --bin uechosearch
make post           # cargo run --bin uechopost
make bench          # cargo run --bin uechobench
make mono           # cargo run --example monolight
```

**Tests must always run with `--test-threads=1`** — parallel test execution causes network race conditions.

## Architecture

Three-layer architecture:

1. **Protocol layer** (`src/protocol/`) — ECHONET-Lite message encoding/decoding, ESV codes (service types like Read/Write/Notify), TID (transaction IDs), property encoding.

2. **Transport layer** (`src/transport/`) — UDP socket management, unicast (direct) and multicast (discovery) communication, notification routing, observer pattern for message callbacks.

3. **Application layer** (`src/`) — `Controller` (search/control remote nodes), `Device` (implement device behavior), `Node` (base container, thread-safe via `Arc<Mutex<T>>`), `Object` (ECHONET device/profile), `Property` (device properties with access rules and type info).

### Key types

- `ObjectCode` — u32 (3 bytes: group class instance); identifies a device type
- `ESV` — service code enum with 16 variants (Read/Write/Notify × Request/Response/Error)
- `Message` — encoded packet: EHD, TID, sender/receiver codes, properties
- `StandardDatabase` — lazy singleton loaded from MRA (Machine Readable Appendix) data

### Database files

`src/database_mra_objects.rs` (~1.3MB) and `src/database_manufacturers.rs` are **auto-generated** from the official MRA standard files in `src/std/MRA_en_v1.3.0/`. Do not hand-edit these.

### Features / platforms

| Feature | Platform |
|---------|----------|
| `unix` (default) | Unix/Linux — uses `pnet`, `nix`, `net2` |
| `win` | Windows — uses `pnet`, `net2` |
| `esp` | ESP32 embedded — uses `esp-idf-sys` |
| `no_std` | No standard library (embedded) |

### Concurrency model

- Thread safety via `Arc<Mutex<T>>` throughout
- Message channels for async communication
- Single-threaded test execution required

## Typical usage pattern

```rust
// Controller
let node = Node::new();
let controller = Controller::new(node);
controller.start()?;
controller.search()?;
let nodes = controller.nodes();

// Device — implement RequestHandler trait
struct MyDevice;
impl RequestHandler for MyDevice { ... }
let device = Device::new(node);
device.start()?;
```
