# μDCN in Rust

This repository contains a minimal work-in-progress implementation of micro Data-Centric Networking (μDCN) written in Rust. The codebase is organized as a Cargo workspace with multiple crates:

- **udcn-common** – shared packet types and utilities.
- **udcn-cli** – simple command line interface.
- **udcn-daemon** – forwarder daemon (stub).
- **udcn-ebpf** – eBPF XDP program.
- **udcn-quic** – QUIC transport abstraction.

Additional folders include `docker/` for container files and `demo/` for example scenarios.

For instructions on building the eBPF program and attaching it using Aya's `XdpManager`, see [`docs/ebpf_build.md`](docs/ebpf_build.md).

All crates currently provide only minimal functionality but compile successfully with `cargo build --workspace`.

## Usage Examples

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

