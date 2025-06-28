use clap::{Parser, Subcommand};
use std::io::{self, BufRead, Write};
use std::os::unix::net::UnixStream;
use std::process::Command;

/// Path to the daemon's Unix socket.
const SOCKET_PATH: &str = "/tmp/udcn.sock";

#[derive(Parser)]
#[command(author, version, about = "μDCN command line interface")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Express an interest for a named data object
    Interest { name: String },
    /// Publish named data (payload not yet implemented)
    Publish { name: String },
    /// Manage the Forwarding Information Base
    Fib {
        #[command(subcommand)]
        command: FibCommand,
    },
    /// Display daemon statistics
    Stats,
    /// Start the forwarder daemon
    Start,
}

#[derive(Subcommand)]
pub enum FibCommand {
    /// Add a prefix to the FIB
    Add { prefix: String },
}

/// Entry point for CLI processing.
pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Interest { name } => {
            let msg = format!("interest {name}\n");
            if let Err(e) = send_daemon(msg) {
                eprintln!("failed to send interest: {e}");
            }
        }
        Commands::Publish { name } => {
            let msg = format!("publish {name}\n");
            if let Err(e) = send_daemon(msg) {
                eprintln!("failed to publish data: {e}");
            }
        }
        Commands::Fib { command } => match command {
            FibCommand::Add { prefix } => {
                // map to a fixed face identifier for demo purposes
                let msg = format!("fib add {prefix} local\n");
                if let Err(e) = send_daemon(msg) {
                    eprintln!("failed to add FIB entry: {e}");
                }
            }
        },
        Commands::Stats => {
            if let Err(e) = send_daemon("cs stats\n".to_string()) {
                eprintln!("failed to request stats: {e}");
            }
        }
        Commands::Start => {
            if let Err(e) = Command::new("udcn-daemon").spawn() {
                eprintln!("failed to start daemon: {e}");
            }
        }
    }
}

fn send_daemon(msg: String) -> io::Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    stream.write_all(msg.as_bytes())?;
    let mut resp = String::new();
    std::io::BufReader::new(&mut stream).read_line(&mut resp)?;
    print!("{resp}");
    Ok(())
}
