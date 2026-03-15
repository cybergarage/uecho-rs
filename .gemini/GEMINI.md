# Gemini Instructions for uecho-rs (echonet)

This project is the Rust implementation of the ECHONET-Lite framework, providing tools to build ECHONET-Lite controllers and devices.

## Project Overview

- **Name:** `echonet` (published as `echonet` crate, source as `uecho-rs`).
- **Core Technology:** Rust (Edition 2024), supporting both `std` and `no_std`.
- **Purpose:** Provide a portable, cross-platform framework for ECHONET-Lite development.
- **Key Abstractions:**
  - `Node`: Represents an ECHONET-Lite node containing objects (profiles and devices).
  - `Device`: Framework for implementing standard ECHONET-Lite devices.
  - `Controller`: Used to discover and control ECHONET-Lite nodes/devices on a network.
  - `Object` & `Property`: Core data structures representing ECHONET-Lite entities and their attributes.
  - `Protocol`: Handles encoding/decoding of ECHONET-Lite messages (ESV, TID, SEOJ, DEOJ, etc.).
  - `Transport`: Manages UDP unicast and multicast communication (default port 3610).

## Building and Running

Common development tasks are facilitated by `cargo` and a `Makefile`.

### Core Commands
- **Build:** `cargo build` (or `make build`)
- **Test:** `cargo test -- --test-threads=1` (or `make test`). 
  - **Note:** Always use `--test-threads=1` to avoid UDP port conflicts between concurrent tests.
- **Format:** `cargo fmt` (or `make format`)
- **Documentation:** `cargo doc --open` (or `make doc`)

### Tools and Examples
- **Discovery Tool:** `cargo run --bin uechosearch` (or `make search`) - Searches for nodes on the network.
- **Messaging Tool:** `cargo run --bin uechopost` (or `make post`) - Sends ECHONET-Lite messages.
- **Benchmark:** `cargo run --bin uechobench` (or `make bench`) - Performs messaging benchmarks.
- **Example Device:** `cargo run --example monolight` (or `make mono`) - A sample Mono Light device.

## Development Conventions

- **Language Standards:** Strict adherence to Rust idiomatic patterns. Uses `edition = "2024"`.
- **Concurrency:** Core components like `Node`, `Device`, and `Controller` are often wrapped in `Arc<Mutex<T>>` for thread safety.
- **Request Handling:** Implement the `RequestHandler` trait to respond to ECHONET-Lite properties requests (Read/Write/Notify).
- **Network Safety:** Tests must be run sequentially because they bind to the standard ECHONET-Lite UDP port (3610).
- **MRA Integration:** The project includes a machine-readable appendix (MRA) database for standard device objects and properties.
- **Feature Flags:**
  - `std`: Enabled by default.
  - `no_std`: Used for embedded environments (e.g., `esp` feature).
  - `unix` / `win`: Platform-specific transport implementations.

## Key Files
- `src/lib.rs`: Public API entry point.
- `src/node.rs`: Core node logic and message routing.
- `src/controller.rs`: Controller-specific logic for node discovery.
- `src/device.rs`: Device framework and standard property management.
- `src/protocol/message.rs`: ECHONET-Lite message structure.
- `src/transport/`: UDP socket and server implementations.
- `examples/monolight/monolight.rs`: Reference implementation of a device.
- `bin/`: CLI utilities for network interaction.
