use std::{collections::HashMap, path::PathBuf};

/// A new in memory key-value store
#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
}

type Result<T> = std::result::Result<T, ()>;

impl Default for KvStore {
    fn default() -> Self {
        KvStore::new()
    }
}

impl KvStore {
    /// Create a new KvStore
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    /// Set a new entry to the KvStore
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("key"), String::from("value"));
    /// assert!(store.get(String::from("key")).is_some());
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }
    /// Get a value from the KvStore by specifying the key
    /// Returns the Ok(value) or [`None`] if the key does not exist
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// let option = store.get(String::from("key"));
    /// assert!(option.is_none());
    /// ```
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).map(|str_slice| str_slice.to_string()))
    }
    /// Remove a value from the KvStore
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("key"), String::from("value"));
    /// store.remove(String::from("key"));
    /// assert!(store.get(String::from("key")).is_none());
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    /// TODO:
    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore::default())
    }
}
