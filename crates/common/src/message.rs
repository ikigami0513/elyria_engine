use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    data: HashMap<String, String>
}

impl Message {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    pub fn get_data(&self) -> HashMap<String, String> {
        self.data.clone()
    }
}