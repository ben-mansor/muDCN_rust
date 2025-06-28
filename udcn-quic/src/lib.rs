//! Placeholder QUIC abstraction.

pub struct QuicTransport;

impl QuicTransport {
    pub fn new() -> Self {
        QuicTransport
    }
}

impl Default for QuicTransport {
    fn default() -> Self {
        Self::new()
    }
}
