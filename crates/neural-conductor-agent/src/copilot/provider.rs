//! # GitHub Copilot Provider
//!
//! Integrates GitHub Copilot into the Neural Conductor agent.
//! Acts as a VSCode extension to access Copilot's API endpoints.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::storage::{StoredAuth, TokenStorage};

/// GitHub Copilot authentication information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotAuth {
    /// OAuth refresh token (long-lived, starts with ghu_)
    pub refresh_token: String,

    /// Copilot session token (short-lived, semicolon-separated key-value pairs)
    pub session_token: String,

    /// Session token expiration timestamp (Unix timestamp in seconds)
    pub expires_at: u64,

    /// Optional enterprise URL
    pub enterprise_url: Option<String>,
}

impl CopilotAuth {
    /// Check if the session token is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now >= self.expires_at
    }

    /// Get the API base URL based on whether this is enterprise or public GitHub
    pub fn base_url(&self) -> String {
        if let Some(enterprise_url) = &self.enterprise_url {
            let domain = Self::normalize_domain(enterprise_url);
            format!("https://copilot-api.{}", domain)
        } else {
            "https://api.githubcopilot.com".to_string()
        }
    }

    /// Get the token refresh URL
    pub fn refresh_url(&self) -> String {
        if let Some(enterprise_url) = &self.enterprise_url {
            let domain = Self::normalize_domain(enterprise_url);
            format!("https://api.{}/copilot_internal/v2/token", domain)
        } else {
            "https://api.github.com/copilot_internal/v2/token".to_string()
        }
    }

    /// Normalize domain by removing protocol and trailing slash
    fn normalize_domain(url: &str) -> String {
        url.replace("https://", "")
            .replace("http://", "")
            .trim_end_matches('/')
            .to_string()
    }
}

/// GitHub Copilot provider
pub struct CopilotProvider {
    auth: CopilotAuth,
    http_client: reqwest::Client,
    storage: TokenStorage,
}

impl CopilotProvider {
    /// Create a new Copilot provider with existing authentication
    pub fn new(auth: CopilotAuth) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .user_agent("GitHubCopilotChat/0.32.4")
            .build()
            .context("Failed to create HTTP client")?;

        let storage = TokenStorage::new()?;

        Ok(Self {
            auth,
            http_client,
            storage,
        })
    }

    /// Load provider from stored credentials
    pub fn from_storage() -> Result<Self> {
        let storage = TokenStorage::new()?;
        let stored = storage.load()?;

        let auth = CopilotAuth {
            refresh_token: stored.github_token,
            session_token: stored.copilot_token,
            expires_at: stored.expires_at,
            enterprise_url: None, // TODO: Store this in StoredAuth
        };

        Self::new(auth)
    }

    /// Save current authentication to storage
    pub fn save_to_storage(&self) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let stored = StoredAuth {
            github_token: self.auth.refresh_token.clone(),
            copilot_token: self.auth.session_token.clone(),
            expires_at: self.auth.expires_at,
            refresh_in: None, // TODO: Track this
            updated_at: now,
        };

        self.storage.save(&stored)
    }

    /// Refresh the session token if expired
    pub async fn ensure_valid_token(&mut self) -> Result<()> {
        if !self.auth.is_expired() {
            return Ok(());
        }

        println!("🔄 Session token expired, refreshing...");

        let refresh_url = self.auth.refresh_url();
        let response = self
            .http_client
            .get(&refresh_url)
            .header("Accept", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.auth.refresh_token),
            )
            .header("Editor-Version", "vscode/1.105.1")
            .header("Editor-Plugin-Version", "copilot-chat/0.32.4")
            .header("Copilot-Integration-Id", "vscode-chat")
            .send()
            .await
            .context("Failed to refresh Copilot token")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Token refresh failed: {} - {}", status, error_text));
        }

        #[derive(Deserialize)]
        struct TokenResponse {
            token: String,
            expires_at: u64,
        }

        let token_data: TokenResponse = response
            .json()
            .await
            .context("Failed to parse token response")?;

        self.auth.session_token = token_data.token;
        self.auth.expires_at = token_data.expires_at;

        // Save updated token to storage
        self.save_to_storage()?;

        println!("✅ Token refreshed, expires at {}", token_data.expires_at);

        Ok(())
    }

    /// Send a chat completion request
    pub async fn chat_completion(&mut self, request: ChatRequest) -> Result<ChatResponse> {
        // Ensure we have a valid token
        self.ensure_valid_token().await?;

        let url = format!("{}/chat/completions", self.auth.base_url());

        let response = self
            .http_client
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.auth.session_token),
            )
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Editor-Version", "vscode/1.105.1")
            .header("Editor-Plugin-Version", "copilot-chat/0.32.4")
            .header("Copilot-Integration-Id", "vscode-chat")
            .json(&request)
            .send()
            .await
            .context("Failed to send chat completion request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Chat completion failed: {} - {}",
                status,
                error_text
            ));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse chat response")?;

        Ok(chat_response)
    }

    /// Get authentication reference
    pub fn auth(&self) -> &CopilotAuth {
        &self.auth
    }
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String, // "system", "user", "assistant"
    pub content: String,
}

/// Chat completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Prompt cache key for session continuity
    #[serde(skip_serializing_if = "Option::is_none", rename = "promptCacheKey")]
    pub prompt_cache_key: Option<String>,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
}

/// Chat choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: String,
}

/// Chat completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,
}

/// Model information with multiplier
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub multiplier: f32,
    pub tier: ModelTier,
}

/// Model pricing tier
#[derive(Debug, Clone, PartialEq)]
pub enum ModelTier {
    Included, // Free on paid plans (0x multiplier)
    Budget,   // Low cost (< 1x multiplier)
    Standard, // Standard cost (1x multiplier)
    Premium,  // High cost (> 1x multiplier)
}

impl ModelInfo {
    /// Get list of available models with their multipliers
    pub fn available_models() -> Vec<ModelInfo> {
        vec![
            // Included models (free on paid plans)
            ModelInfo {
                name: "gpt-4o".to_string(),
                multiplier: 0.0,
                tier: ModelTier::Included,
            },
            ModelInfo {
                name: "gpt-4.1".to_string(),
                multiplier: 0.0,
                tier: ModelTier::Included,
            },
            ModelInfo {
                name: "gpt-5-mini".to_string(),
                multiplier: 0.0,
                tier: ModelTier::Included,
            },
            ModelInfo {
                name: "raptor-mini".to_string(),
                multiplier: 0.0,
                tier: ModelTier::Included,
            },
            // Budget models
            ModelInfo {
                name: "grok-code-fast-1".to_string(),
                multiplier: 0.25,
                tier: ModelTier::Budget,
            },
            ModelInfo {
                name: "claude-haiku-4.5".to_string(),
                multiplier: 0.33,
                tier: ModelTier::Budget,
            },
            ModelInfo {
                name: "gpt-5.1-codex-mini".to_string(),
                multiplier: 0.33,
                tier: ModelTier::Budget,
            },
            // Standard models
            ModelInfo {
                name: "claude-sonnet-4".to_string(),
                multiplier: 1.0,
                tier: ModelTier::Standard,
            },
            ModelInfo {
                name: "claude-sonnet-4.5".to_string(),
                multiplier: 1.0,
                tier: ModelTier::Standard,
            },
            ModelInfo {
                name: "gemini-2.5-pro".to_string(),
                multiplier: 1.0,
                tier: ModelTier::Standard,
            },
            ModelInfo {
                name: "gemini-3-pro".to_string(),
                multiplier: 1.0,
                tier: ModelTier::Standard,
            },
            ModelInfo {
                name: "gpt-5".to_string(),
                multiplier: 1.0,
                tier: ModelTier::Standard,
            },
            ModelInfo {
                name: "gpt-5.1".to_string(),
                multiplier: 1.0,
                tier: ModelTier::Standard,
            },
            ModelInfo {
                name: "gpt-5.1-codex".to_string(),
                multiplier: 1.0,
                tier: ModelTier::Standard,
            },
            ModelInfo {
                name: "gpt-5.1-codex-max".to_string(),
                multiplier: 1.0,
                tier: ModelTier::Standard,
            },
            ModelInfo {
                name: "gpt-5.2".to_string(),
                multiplier: 1.0,
                tier: ModelTier::Standard,
            },
            // Premium models
            ModelInfo {
                name: "claude-opus-4.5".to_string(),
                multiplier: 3.0,
                tier: ModelTier::Premium,
            },
            ModelInfo {
                name: "claude-opus-4.1".to_string(),
                multiplier: 10.0,
                tier: ModelTier::Premium,
            },
        ]
    }

    /// Find model info by name
    pub fn find(model_name: &str) -> Option<ModelInfo> {
        Self::available_models()
            .into_iter()
            .find(|m| m.name == model_name)
    }

    /// Calculate premium requests for a number of prompts
    pub fn calculate_premium_requests(&self, prompt_count: u32) -> f32 {
        prompt_count as f32 * self.multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_expiration() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let auth = CopilotAuth {
            refresh_token: "ghu_test".to_string(),
            session_token: "tid=test;exp=123".to_string(),
            expires_at: now - 100, // Expired 100 seconds ago
            enterprise_url: None,
        };

        assert!(auth.is_expired());

        let auth2 = CopilotAuth {
            expires_at: now + 3600, // Expires in 1 hour
            ..auth
        };

        assert!(!auth2.is_expired());
    }

    #[test]
    fn test_base_url() {
        let auth = CopilotAuth {
            refresh_token: "ghu_test".to_string(),
            session_token: "tid=test".to_string(),
            expires_at: 999999999,
            enterprise_url: None,
        };

        assert_eq!(auth.base_url(), "https://api.githubcopilot.com");

        let enterprise_auth = CopilotAuth {
            enterprise_url: Some("https://github.company.com".to_string()),
            ..auth
        };

        assert_eq!(
            enterprise_auth.base_url(),
            "https://copilot-api.github.company.com"
        );
    }

    #[test]
    fn test_model_multipliers() {
        let gpt4o = ModelInfo::find("gpt-4o").unwrap();
        assert_eq!(gpt4o.multiplier, 0.0);
        assert_eq!(gpt4o.tier, ModelTier::Included);

        let sonnet = ModelInfo::find("claude-sonnet-4.5").unwrap();
        assert_eq!(sonnet.multiplier, 1.0);
        assert_eq!(sonnet.tier, ModelTier::Standard);

        let opus = ModelInfo::find("claude-opus-4.1").unwrap();
        assert_eq!(opus.multiplier, 10.0);
        assert_eq!(opus.tier, ModelTier::Premium);
    }

    #[test]
    fn test_premium_request_calculation() {
        let gpt4o = ModelInfo::find("gpt-4o").unwrap();
        assert_eq!(gpt4o.calculate_premium_requests(100), 0.0);

        let sonnet = ModelInfo::find("claude-sonnet-4.5").unwrap();
        assert_eq!(sonnet.calculate_premium_requests(5), 5.0);

        let opus = ModelInfo::find("claude-opus-4.1").unwrap();
        assert_eq!(opus.calculate_premium_requests(3), 30.0);
    }
}
