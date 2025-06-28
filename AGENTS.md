🎯 Objective
Create a clean, minimal, and working implementation of μDCN (Micro Data-Centric Networking) in Rust, including:

A simplified architecture combining:

Rust-based CLI

QUIC transport layer

eBPF/XDP packet processor (must compile and attach)

Named Data Networking (NDN) logic

Working containerized testbed (using Docker)

A modular, testable codebase with proper daemonization and IPC

Clear separation of responsibilities between CLI, forwarder, and simulation

🧠 Agent Role: codex/agent-core
💼 Responsibilities
The Codex Agent must:

Create a new Rust project with proper workspace layout

Build a minimal eBPF XDP program (that compiles and attaches)

Create a forwarder daemon using async Rust (Tokio)

Provide CLI commands using clap to communicate via Unix sockets

Implement QUIC support using quinn

Package it into a working Docker testbed (3 nodes: client, forwarder, server)

Provide a documented simulation + manual testing mode

🏗️ Project Layout
bash
Copy
Edit
rust-udcn/
├── Cargo.toml (workspace)
├── udcn-cli/                # CLI binary crate
├── udcn-daemon/             # Daemon with persistent forwarding state
├── udcn-ebpf/               # eBPF crate (XDP forwarding program)
├── udcn-common/             # Shared structs, NDN types, config
├── udcn-quic/               # QUIC transport abstraction
├── docker/                  # Docker files, start scripts
├── demo/                    # Test scenarios, validation scripts
└── README.md
✅ Required Features
1. 🧠 Core Logic
Minimal Interest/Data packet struct (Name, Nonce, Payload)

In-memory FIB, PIT, CS data structures

Longest prefix matching (FIB)

PIT with expiration and duplicate suppression

CS with simple LRU cache

2. 🚀 eBPF XDP
Standalone udcn-ebpf crate

Must compile with:

sql
Copy
Edit
rustup default nightly
rustup target add bpfel-unknown-none
cargo build -p udcn-ebpf --target=bpfel-unknown-none -Z build-std=core
Attach via XdpManager using aya or redbpf

3. 🔧 CLI
udcn interest /name

udcn publish /name

udcn fib add /prefix

udcn stats

udcn start (starts the forwarder as a daemon)

4. 🧪 Testing/Demo
Scripted setup:

docker-compose.yml

test_udcn.sh to launch client/server/forwarder

Sample Interest/Data exchange and log output

Logs, stats, and runtime metrics via CLI

📁 Output Files
README.md – full usage guide

persistent-daemon.md – daemon IPC interface spec

testbed.md – Docker testbed + expected output

validation_report.txt – log summary of passing test/demo

🧼 Quality Constraints
No broken commands or placeholder logic

No fake output — real packet flow must be tested in Docker

No excessive simulations in place of real forwarder

All binaries must compile with cargo build --workspace

Use stable crates and documented dependencies

Minimal warnings in CI: cargo check, clippy, fmt

📅 Deliverables
Initial working prototype: eBPF + CLI + testbed

Fully documented CLI: help text, examples

Sample Docker demo running in 3 terminals

End-to-end working Interest/Data exchange

Ability to publish, cache, and retrieve content
