//! Session management

use crate::Result;
use neural_conductor_shared::{SessionId, session::Session};
use std::collections::HashMap;

/// Manages active sessions
pub struct SessionManager {
    sessions: HashMap<SessionId, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
    
    pub fn create_session(&mut self, id: SessionId, workspace_path: String) -> Result<()> {
        let session = Session::new(id.clone(), workspace_path);
        self.sessions.insert(id, session);
        Ok(())
    }
    
    pub fn terminate_session(&mut self, id: &SessionId) -> Result<()> {
        self.sessions.remove(id);
        Ok(())
    }
    
    pub fn get_session(&self, id: &SessionId) -> Option<&Session> {
        self.sessions.get(id)
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
