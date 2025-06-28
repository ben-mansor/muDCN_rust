use lru::LruCache;
use udcn_common::Data;

/// Content Store implementing an LRU cache.
pub struct ContentStore {
    cache: LruCache<String, Data>,
}

impl ContentStore {
    /// Create a new content store with the given capacity.
    pub fn new(capacity: usize) -> Self {
        ContentStore {
            cache: LruCache::new(capacity.max(16)),
        }
    }

    /// Insert a Data packet into the store.
    pub fn insert(&mut self, data: Data) {
        self.cache.put(data.name.clone(), data);
    }

    /// Retrieve a Data packet by name.
    pub fn get(&mut self, name: &str) -> Option<&Data> {
        self.cache.get(name)
    }

    /// Return current number of stored entries.
    pub fn len(&self) -> usize {
        self.cache.len()
    }
}
