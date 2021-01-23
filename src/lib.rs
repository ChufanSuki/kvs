#![deny(missing_docs)]
//! # kvs
//!
//! Defines `KvStore` struct which implements a simple in-memory key-value storage
use std::collections::HashMap;

/// A key-value store is represented here
pub struct KvStore {
    kvs: HashMap<String, String>,
}

impl KvStore {
    /// Return a new KvStore
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store: KvStore = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    ///
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            kvs: HashMap::new(),
        }
    }
    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) {
        self.kvs.insert(key, value);
    }
    /// Get the string value of the a string key. If the key does not exist, return None.
    pub fn get(&self, key: String) -> Option<String> {
        self.kvs.get(&key).map(|value| value.clone())
    }
    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        self.kvs.remove(&key).unwrap();
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
