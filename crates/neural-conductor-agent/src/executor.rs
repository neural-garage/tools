//! Command execution module

use crate::Result;
use neural_conductor_shared::SessionId;
use std::process::Command;

/// Execute a command and return the result
pub fn execute_command(
    session_id: &SessionId,
    command: &str,
    args: &[String],
    workdir: Option<&str>,
) -> Result<(i32, String, String)> {
    let mut cmd = Command::new(command);
    cmd.args(args);
    
    if let Some(dir) = workdir {
        cmd.current_dir(dir);
    }
    
    let output = cmd.output()?;
    
    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    Ok((exit_code, stdout, stderr))
}
