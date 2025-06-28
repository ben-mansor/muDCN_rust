# Building and Attaching the eBPF Program

The `udcn-ebpf` crate is a `no_std` XDP program. Building requires the
nightly toolchain and the `bpfel-unknown-none` target.

```bash
rustup default nightly
rustup target add bpfel-unknown-none
```

Build the program with:

```bash
cargo build -p udcn-ebpf --target=bpfel-unknown-none -Z build-std=core
```

The resulting object file can be loaded using `XdpManager` from the
`aya` project:

```rust
use aya::Bpf;
use aya::programs::Xdp;
use aya::util::online_cpus;
use aya::maps::ProgramData;

let mut bpf = Bpf::load_file("target/bpfel-unknown-none/debug/udcn-ebpf")?;
let program: &mut Xdp = bpf.program_mut("xdp_pass")?.try_into()?;
program.load()?;
program.attach("eth0", aya::programs::XdpFlags::default())?;
```

This attaches the XDP program to the interface `eth0` and simply passes
all packets through.
