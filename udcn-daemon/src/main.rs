use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

const SOCKET_PATH: &str = "/tmp/udcn.sock";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("udcn-daemon starting...");
    let _ = std::fs::remove_file(SOCKET_PATH);
    let listener = UnixListener::bind(SOCKET_PATH)?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("client error: {e}");
            }
        });
    }
}

async fn handle_client(stream: UnixStream) -> std::io::Result<()> {
    let (r, mut w) = stream.into_split();
    let mut lines = BufReader::new(r).lines();
    while let Some(line) = lines.next_line().await? {
        println!("daemon received: {line}");
        w.write_all(b"ok\n").await?;
    }
    Ok(())
}
