//! # Neural Conductor Shared
//!
//! Shared protocol and types for Neural Conductor.
//!
//! This crate provides the communication protocol between the Conductor server
//! and remote agents. It defines message types, session management, and RPC interfaces.

use serde::{Deserialize, Serialize};

pub mod message;
pub mod protocol;
pub mod session;

pub use anyhow::{anyhow, Result};

/// Version of the protocol
pub const PROTOCOL_VERSION: &str = "0.1.0";

/// Agent identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub hostname: String,
    pub platform: String,
    pub version: String,
}

/// Session identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SessionId(pub String);

impl SessionId {
    pub fn new() -> Self {
        // Simple UUID-like generation for now
        Self(format!(
            "session-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ))
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}
