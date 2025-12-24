//! GitHub Copilot integration for Neural Conductor
//!
//! Provides authentication, token management, and API access to GitHub Copilot
//! for LLM-powered code analysis and generation.

pub mod auth;
pub mod provider;
pub mod storage;

pub use auth::DeviceFlowAuth;
pub use provider::{CopilotProvider, ModelInfo};
pub use storage::{StoredAuth, TokenStorage};
