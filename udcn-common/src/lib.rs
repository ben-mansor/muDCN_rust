//! Common types for the μDCN project.

/// A minimal Interest packet.
#[derive(Debug, Clone)]
pub struct Interest {
    pub name: String,
    pub nonce: u64,
}

/// A minimal Data packet.
#[derive(Debug, Clone)]
pub struct Data {
    pub name: String,
    pub payload: Vec<u8>,
}

