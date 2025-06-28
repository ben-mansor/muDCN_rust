use std::collections::HashMap;

/// Forwarding Information Base with longest prefix match.
#[derive(Default)]
pub struct Fib {
    entries: HashMap<String, String>, // prefix -> face identifier
}

impl Fib {
    /// Add a prefix and next hop identifier.
    pub fn add(&mut self, prefix: String, face: String) {
        self.entries.insert(prefix, face);
    }

    /// Return the next hop that best matches the name.
    pub fn lookup(&self, name: &str) -> Option<&String> {
        let mut best: Option<&String> = None;
        let mut best_len = 0usize;
        for (prefix, face) in &self.entries {
            if name.starts_with(prefix) && prefix.len() > best_len {
                best = Some(face);
                best_len = prefix.len();
            }
        }
        best
    }

    pub fn list(&self) -> Vec<(String, String)> {
        self.entries
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}
