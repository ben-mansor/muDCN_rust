use quinn::{Connection, Endpoint};
use udcn_common::{Data, Interest};

/// QUIC transport wrapper built on top of `quinn`.
pub struct QuicTransport {
    endpoint: Endpoint,
    connection: Option<Connection>,
}

impl Default for QuicTransport {
    fn default() -> Self {
        Self::new().expect("failed to create QuicTransport")
    }
}

impl QuicTransport {
    /// Create a new client-only transport bound to an ephemeral UDP port.
    pub fn new() -> Result<Self, std::io::Error> {
        let endpoint = Endpoint::client("0.0.0.0:0".parse().unwrap())?;
        Ok(Self {
            endpoint,
            connection: None,
        })
    }

    /// Establish a connection to the given peer address.
    pub async fn connect(&mut self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr: std::net::SocketAddr = addr.parse().expect("invalid address");
        let connecting = self.endpoint.connect(addr, "udcn")?;
        let conn = connecting.await?;
        self.connection = Some(conn);
        Ok(())
    }

    async fn send_raw(&self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.connection.as_ref().ok_or("not connected")?;
        let mut stream = conn.open_uni().await?;
        stream.write_all(bytes).await?;
        stream
            .finish()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(())
    }

    async fn recv_raw(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let conn = self.connection.as_mut().ok_or("not connected")?;
        let mut stream = conn.accept_uni().await?;
        let data = stream.read_to_end(4096).await?;
        Ok(data)
    }

    /// Send an Interest packet to the connected peer.
    pub async fn send_interest(
        &self,
        interest: &Interest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = serde_json::to_vec(interest)?;
        self.send_raw(&bytes).await
    }

    /// Receive an Interest packet from the peer.
    pub async fn recv_interest(&mut self) -> Result<Interest, Box<dyn std::error::Error>> {
        let bytes = self.recv_raw().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Send a Data packet to the connected peer.
    pub async fn send_data(&self, data: &Data) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = serde_json::to_vec(data)?;
        self.send_raw(&bytes).await
    }

    /// Receive a Data packet from the peer.
    pub async fn recv_data(&mut self) -> Result<Data, Box<dyn std::error::Error>> {
        let bytes = self.recv_raw().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}
