use std::collections::HashMap;

/// A key-value store that owns its data.
/// By wrapping the HashMap in our own struct, we can control the public API.
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new, empty `KvStore`.
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the store.
    /// It takes ownership of both the key and the value.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Retrieves a value from the store, given a key.
    /// It borrows the key (`&str`) for the lookup, avoiding unnecessary allocations.
    /// Returns an `Option<&String>` because the key might not exist.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    /// Removes a key-value pair from the store.
    /// It borrows the key (`&str`) for the lookup.
    /// Returns the old value as an `Option<String>` if the key existed.
    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }
}