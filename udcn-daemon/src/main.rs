mod cs;
mod fib;
mod pit;

use cs::ContentStore;
use fib::Fib;
use pit::Pit;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("udcn-daemon starting...");
    let _ = std::fs::remove_file("/tmp/udcn.sock");
    let listener = UnixListener::bind("/tmp/udcn.sock")?;

    let fib = Arc::new(Mutex::new(Fib::default()));
    let _pit = Arc::new(Mutex::new(Pit::default()));
    let cs = Arc::new(Mutex::new(ContentStore::new(16)));

    loop {
        let (stream, _) = listener.accept().await?;
        let fib = fib.clone();
        let cs = cs.clone();
        tokio::spawn(async move {
            let (r, mut w) = stream.into_split();
            let mut reader = BufReader::new(r);
            let mut line = String::new();
            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                let parts: Vec<String> = line
                    .trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                line.clear();
                if parts.is_empty() {
                    continue;
                }
                match parts[0].as_str() {
                    "fib" => {
                        if parts.get(1).map(|s| s.as_str()) == Some("add") {
                            if let (Some(p), Some(f)) = (parts.get(2), parts.get(3)) {
                                fib.lock().unwrap().add(p.clone(), f.clone());
                                let _ = w.write_all(b"OK\n").await;
                            } else {
                                let _ = w.write_all(b"ERR\n").await;
                            }
                        } else if parts.get(1).map(|s| s.as_str()) == Some("list") {
                            let entries = fib.lock().unwrap().list();
                            for (p, f) in entries {
                                let _ = w.write_all(format!("{} -> {}\n", p, f).as_bytes()).await;
                            }
                        }
                    }
                    "cs" => {
                        if parts.get(1).map(|s| s.as_str()) == Some("stats") {
                            let len = cs.lock().unwrap().len();
                            let _ = w
                                .write_all(format!("CS entries: {}\n", len).as_bytes())
                                .await;
                        }
                    }
                    "quit" => break,
                    _ => {
                        let _ = w.write_all(b"UNKNOWN\n").await;
                    }
                }
            }
        });
    }
}
