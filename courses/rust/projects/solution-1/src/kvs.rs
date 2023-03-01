use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        self.map.get(&key).map(|str_slice| str_slice.to_string())
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
