# μDCN in Rust

This repository contains a minimal work-in-progress implementation of micro Data-Centric Networking (μDCN) written in Rust. The codebase is organized as a Cargo workspace with multiple crates:

- **udcn-common** – shared packet types and utilities.
- **udcn-cli** – simple command line interface.
- **udcn-daemon** – forwarder daemon (stub).
- **udcn-ebpf** – XDP program placeholder.
- **udcn-quic** – QUIC transport abstraction.

Additional folders include `docker/` for container files and `demo/` for example scenarios.

All crates currently provide only minimal functionality but compile successfully with `cargo build --workspace`.

