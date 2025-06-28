use std::collections::HashMap;
use tokio::time::{Duration, Instant};

/// Pending Interest Table entry expiration time.
pub type Expiration = Instant;

/// Pending Interest Table.
#[derive(Default)]
pub struct Pit {
    entries: HashMap<String, Expiration>,
}

impl Pit {
    /// Insert a name with a lifetime.
    pub fn insert(&mut self, name: String, lifetime: Duration) {
        self.entries.insert(name, Instant::now() + lifetime);
    }

    /// Remove an entry.
    pub fn remove(&mut self, name: &str) {
        self.entries.remove(name);
    }

    /// Check if a name exists.
    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }

    /// Drop expired entries.
    pub fn sweep(&mut self) {
        let now = Instant::now();
        self.entries.retain(|_, &mut exp| exp > now);
    }
}
