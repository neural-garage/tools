//! Protocol definitions for Conductor communication

use super::{SessionId, TaskStatus};
use serde::{Deserialize, Serialize};

/// Request from server to agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    /// Ping to check agent health
    Ping,

    /// Execute a command in a session
    ExecuteCommand {
        session_id: SessionId,
        command: String,
        args: Vec<String>,
        workdir: Option<String>,
    },

    /// Create a new session
    CreateSession {
        session_id: SessionId,
        workspace_path: String,
    },

    /// Terminate a session
    TerminateSession { session_id: SessionId },

    /// Get session status
    GetSessionStatus { session_id: SessionId },
}

/// Response from agent to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    /// Pong response
    Pong { agent_info: super::AgentInfo },

    /// Command execution result
    CommandResult {
        session_id: SessionId,
        exit_code: i32,
        stdout: String,
        stderr: String,
    },

    /// Session created
    SessionCreated { session_id: SessionId },

    /// Session terminated
    SessionTerminated { session_id: SessionId },

    /// Session status
    SessionStatus {
        session_id: SessionId,
        status: TaskStatus,
    },

    /// Error response
    Error { message: String },
}
