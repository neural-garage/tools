//! Message framing and serialization

use super::Result;
use serde::{Deserialize, Serialize};

/// Wrapper for protocol messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T> {
    pub version: String,
    pub timestamp: u64,
    pub payload: T,
}

impl<T: Serialize> Message<T> {
    pub fn new(payload: T) -> Self {
        Self {
            version: super::PROTOCOL_VERSION.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            payload,
        }
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl<T: for<'de> Deserialize<'de>> Message<T> {
    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}
