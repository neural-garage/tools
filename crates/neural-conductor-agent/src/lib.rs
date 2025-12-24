//! # Neural Conductor Agent
//!
//! Remote agent for Neural Conductor orchestration platform.
//!
//! This agent runs on remote machines and executes commands on behalf of
//! the Conductor server. It manages sessions, executes commands, and
//! reports results back to the server.

pub mod executor;
pub mod session_manager;

pub use neural_conductor_shared::{
    AgentInfo, SessionId, TaskStatus,
    protocol::{Request, Response},
};

pub use anyhow::{anyhow, Result};

/// Agent version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Agent runtime
pub struct Agent {
    info: AgentInfo,
}

impl Agent {
    pub fn new() -> Self {
        let hostname = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string());
        
        Self {
            info: AgentInfo {
                id: format!("agent-{}", hostname),
                hostname,
                platform: std::env::consts::OS.to_string(),
                version: VERSION.to_string(),
            },
        }
    }
    
    pub fn info(&self) -> &AgentInfo {
        &self.info
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}

// Placeholder for hostname detection
mod hostname {
    use std::ffi::OsString;
    pub fn get() -> Result<OsString, ()> {
        // This is a placeholder - in real implementation we'd use gethostname() or similar
        Ok(OsString::from("localhost"))
    }
}
