//! Common types for the μDCN project.

/// A minimal Interest packet.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interest {
    pub name: String,
    pub nonce: u64,
}

/// A minimal Data packet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub payload: Vec<u8>,
}
