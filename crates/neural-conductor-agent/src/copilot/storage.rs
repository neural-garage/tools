//! Secure token storage for GitHub Copilot credentials
//!
//! Handles persistence of authentication tokens with proper file permissions.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Stored authentication data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAuth {
    /// GitHub OAuth refresh token (long-lived)
    pub github_token: String,

    /// Copilot session token (short-lived, contains features/endpoints)
    pub copilot_token: String,

    /// Unix timestamp when the Copilot token expires
    pub expires_at: u64,

    /// Optional: when to refresh the token (before expiry)
    pub refresh_in: Option<u64>,

    /// Timestamp when this auth was last updated
    pub updated_at: u64,
}

/// Token storage manager
pub struct TokenStorage {
    storage_path: PathBuf,
}

impl TokenStorage {
    /// Create a new token storage with the default path
    pub fn new() -> Result<Self> {
        let storage_path = Self::default_storage_path()?;
        Ok(Self { storage_path })
    }

    /// Create a new token storage with a custom path
    pub fn with_path(path: PathBuf) -> Self {
        Self { storage_path: path }
    }

    /// Get the default storage path: ~/.config/neural-conductor/copilot-auth.json
    fn default_storage_path() -> Result<PathBuf> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| anyhow!("Could not determine config directory"))?;

        let neural_config = config_dir.join("neural-conductor");
        Ok(neural_config.join("copilot-auth.json"))
    }

    /// Ensure the parent directory exists with proper permissions
    fn ensure_parent_dir(&self) -> Result<()> {
        if let Some(parent) = self.storage_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;

                #[cfg(unix)]
                {
                    // Set directory permissions to 0700 (rwx------)
                    let metadata = fs::metadata(parent)?;
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o700);
                    fs::set_permissions(parent, permissions)?;
                }
            }
        }
        Ok(())
    }

    /// Save authentication data to disk with secure permissions
    pub fn save(&self, auth: &StoredAuth) -> Result<()> {
        self.ensure_parent_dir()?;

        let json = serde_json::to_string_pretty(auth).context("Failed to serialize auth data")?;

        // Write to a temporary file first
        let temp_path = self.storage_path.with_extension("tmp");
        fs::write(&temp_path, json)
            .with_context(|| format!("Failed to write to {}", temp_path.display()))?;

        #[cfg(unix)]
        {
            // Set file permissions to 0600 (rw-------)
            let metadata = fs::metadata(&temp_path)?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o600);
            fs::set_permissions(&temp_path, permissions)?;
        }

        // Atomic rename
        fs::rename(&temp_path, &self.storage_path)
            .with_context(|| format!("Failed to save auth to {}", self.storage_path.display()))?;

        Ok(())
    }

    /// Load authentication data from disk
    pub fn load(&self) -> Result<StoredAuth> {
        if !self.storage_path.exists() {
            return Err(anyhow!(
                "No authentication data found. Please run 'neural-conductor-agent copilot login' first."
            ));
        }

        // Check file permissions on Unix
        #[cfg(unix)]
        {
            let metadata = fs::metadata(&self.storage_path)?;
            let permissions = metadata.permissions();
            let mode = permissions.mode() & 0o777;

            if mode & 0o077 != 0 {
                eprintln!(
                    "Warning: Auth file has insecure permissions ({:o}). Should be 0600.",
                    mode
                );
            }
        }

        let contents = fs::read_to_string(&self.storage_path)
            .with_context(|| format!("Failed to read {}", self.storage_path.display()))?;

        serde_json::from_str(&contents).context("Failed to parse stored authentication data")
    }

    /// Delete stored authentication data
    pub fn delete(&self) -> Result<()> {
        if self.storage_path.exists() {
            fs::remove_file(&self.storage_path)
                .with_context(|| format!("Failed to delete {}", self.storage_path.display()))?;
        }
        Ok(())
    }

    /// Check if authentication data exists
    pub fn exists(&self) -> bool {
        self.storage_path.exists()
    }

    /// Get the storage path
    pub fn path(&self) -> &Path {
        &self.storage_path
    }
}

impl Default for TokenStorage {
    fn default() -> Self {
        Self::new().expect("Failed to create default token storage")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_storage_path_creation() {
        let storage = TokenStorage::new().unwrap();
        assert!(storage
            .path()
            .to_string_lossy()
            .contains("neural-conductor"));
        assert!(storage
            .path()
            .to_string_lossy()
            .contains("copilot-auth.json"));
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test-copilot-auth.json");
        let storage = TokenStorage::with_path(test_path.clone());

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let auth = StoredAuth {
            github_token: "ghu_test123".to_string(),
            copilot_token: "test_token".to_string(),
            expires_at: now + 86400,
            refresh_in: Some(43200),
            updated_at: now,
        };

        // Save
        storage.save(&auth).unwrap();
        assert!(test_path.exists());

        // Load
        let loaded = storage.load().unwrap();
        assert_eq!(loaded.github_token, auth.github_token);
        assert_eq!(loaded.copilot_token, auth.copilot_token);

        // Cleanup
        storage.delete().unwrap();
        assert!(!test_path.exists());
    }

    #[cfg(unix)]
    #[test]
    fn test_secure_permissions() {
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test-copilot-perms.json");
        let storage = TokenStorage::with_path(test_path.clone());

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let auth = StoredAuth {
            github_token: "ghu_test".to_string(),
            copilot_token: "test".to_string(),
            expires_at: now + 86400,
            refresh_in: None,
            updated_at: now,
        };

        storage.save(&auth).unwrap();

        // Check permissions are 0600
        let metadata = fs::metadata(&test_path).unwrap();
        let permissions = metadata.permissions();
        let mode = permissions.mode() & 0o777;
        assert_eq!(mode, 0o600, "File should have 0600 permissions");

        // Cleanup
        storage.delete().unwrap();
    }
}
