# μDCN in Rust

This repository contains a minimal implementation of micro Data-Centric Networking (μDCN) written in Rust. The project is organised as a Cargo workspace and consists of multiple crates.

## Architecture Overview

- **udcn-common** – shared packet types and utilities.
- **udcn-cli** – command line interface for interacting with the daemon.
- **udcn-daemon** – async forwarder daemon providing a Unix socket API.
- **udcn-ebpf** – XDP program compiled to eBPF and attached by the daemon.
- **udcn-quic** – QUIC transport abstraction used by future components.

Additional folders:

- **docker/** – container files and compose configuration.
- **demo/** – simple scripts to run a multi node testbed.

## Building

1. Install the Rust toolchain and `cargo`.
2. Build all crates:
   ```bash
   cargo build --workspace
   ```
3. To build the eBPF program you need the nightly toolchain and the `bpfel-unknown-none` target:
   ```bash
   rustup default nightly
   rustup target add bpfel-unknown-none
   cargo build -p udcn-ebpf --target=bpfel-unknown-none -Z build-std=core
   ```
   See [`docs/ebpf_build.md`](docs/ebpf_build.md) for details.

## CLI Examples

Start the daemon:
```bash
$ udcn start
```

Express an interest:
```bash
$ udcn interest /example/data
```

Publish data (payload handling not yet implemented):
```bash
$ udcn publish /example/data
```

Add a FIB entry:
```bash
$ udcn fib add /example
```

Request statistics from the daemon:
```bash
$ udcn stats
```

