# Project Overview

This document briefly describes the layout of the repository. Each crate in the workspace provides a specific piece of the μDCN prototype.

- `udcn-cli` – command line tool that talks to the daemon over a Unix socket.
- `udcn-daemon` – forwarder daemon attaching the eBPF program and maintaining forwarding state.
- `udcn-ebpf` – minimal XDP program compiled with the nightly Rust toolchain.
- `udcn-quic` – experimental QUIC transport wrapper.
- `udcn-common` – shared data structures (Interest and Data packets).

For Docker based experiments consult `docker/README.md` and `demo/README.md`.

