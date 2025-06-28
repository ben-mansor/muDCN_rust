use udcn_common::{Interest, Data};

/// Entry point for CLI processing.
pub fn run() {
    let mut args = std::env::args();
    let _program = args.next();
    match args.next().as_deref() {
        Some("interest") => {
            if let Some(name) = args.next() {
                let interest = Interest { name, nonce: 0 };
                println!("Sending interest: {:?}", interest);
            }
        }
        Some("publish") => {
            if let Some(name) = args.next() {
                let data = Data { name, payload: Vec::new() };
                println!("Publishing data: {:?}", data);
            }
        }
        _ => {
            eprintln!("usage: udcn <interest|publish> <name>");
        }
    }
}

