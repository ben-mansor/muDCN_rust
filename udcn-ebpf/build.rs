use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if target == "bpfel-unknown-none" {
        // Preserve existing flags if any
        let mut flags = env::var("RUSTFLAGS").unwrap_or_default();
        if !flags.is_empty() {
            flags.push(' ');
        }
        flags.push_str("-Z build-std=core");
        println!("cargo:rustc-env=RUSTFLAGS={flags}");
    }
}
