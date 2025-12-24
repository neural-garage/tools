//! GitHub Copilot OAuth Device Flow Authentication
//!
//! Implements the OAuth 2.0 Device Authorization Grant flow for GitHub Copilot.
//! This allows users to authenticate via their browser while the CLI polls for completion.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::time::Duration;

const DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const COPILOT_TOKEN_URL: &str = "https://api.github.com/copilot_internal/v2/token";

// GitHub OAuth App credentials for Copilot
// These are public client credentials (similar to VSCode extension)
const CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";
const SCOPE: &str = "read:user";

/// Device code response from GitHub
#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

/// OAuth access token response
#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

/// Error response during token polling
#[derive(Debug, Deserialize)]
pub struct TokenErrorResponse {
    pub error: String,
    pub error_description: Option<String>,
}

/// Copilot session token response
#[derive(Debug, Deserialize)]
pub struct CopilotTokenResponse {
    pub token: String,
    pub expires_at: u64,
    pub refresh_in: Option<u64>,
}

/// OAuth Device Flow handler
pub struct DeviceFlowAuth {
    client: reqwest::Client,
}

impl DeviceFlowAuth {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Step 1: Request a device code from GitHub
    pub async fn request_device_code(&self) -> Result<DeviceCodeResponse> {
        let response = self
            .client
            .post(DEVICE_CODE_URL)
            .header("Accept", "application/json")
            .form(&[("client_id", CLIENT_ID), ("scope", SCOPE)])
            .send()
            .await
            .context("Failed to request device code")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Device code request failed ({}): {}", status, text));
        }

        response
            .json::<DeviceCodeResponse>()
            .await
            .context("Failed to parse device code response")
    }

    /// Step 2: Poll for access token until user completes authorization
    pub async fn poll_for_token(
        &self,
        device_code: &str,
        interval: u64,
        expires_in: u64,
    ) -> Result<String> {
        let poll_interval = Duration::from_secs(interval);
        let timeout = Duration::from_secs(expires_in);
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err(anyhow!(
                    "Device code expired before authorization completed"
                ));
            }

            tokio::time::sleep(poll_interval).await;

            let response = self
                .client
                .post(ACCESS_TOKEN_URL)
                .header("Accept", "application/json")
                .form(&[
                    ("client_id", CLIENT_ID),
                    ("device_code", device_code),
                    ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ])
                .send()
                .await
                .context("Failed to poll for access token")?;

            let status = response.status();
            let text = response.text().await.unwrap_or_default();

            if status.is_success() {
                // Try to parse as success response
                if let Ok(token_response) = serde_json::from_str::<AccessTokenResponse>(&text) {
                    return Ok(token_response.access_token);
                }
            }

            // Parse error response
            if let Ok(error_response) = serde_json::from_str::<TokenErrorResponse>(&text) {
                match error_response.error.as_str() {
                    "authorization_pending" => {
                        // User hasn't completed auth yet, continue polling
                        continue;
                    }
                    "slow_down" => {
                        // We're polling too fast, wait longer
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                    "expired_token" => {
                        return Err(anyhow!("Device code expired"));
                    }
                    "access_denied" => {
                        return Err(anyhow!("User denied authorization"));
                    }
                    _ => {
                        return Err(anyhow!(
                            "Authentication error: {}",
                            error_response
                                .error_description
                                .unwrap_or(error_response.error)
                        ));
                    }
                }
            }

            // Unknown response format
            return Err(anyhow!("Unexpected response format: {}", text));
        }
    }

    /// Step 3: Exchange GitHub OAuth token for Copilot session token
    pub async fn get_copilot_token(&self, github_token: &str) -> Result<CopilotTokenResponse> {
        let response = self
            .client
            .get(COPILOT_TOKEN_URL)
            .header("Authorization", format!("token {}", github_token))
            .header("Accept", "application/json")
            .header("User-Agent", "neural-conductor-agent")
            .send()
            .await
            .context("Failed to get Copilot token")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();

            if status.as_u16() == 401 {
                return Err(anyhow!(
                    "GitHub token is invalid or expired. Please re-authenticate."
                ));
            }

            if status.as_u16() == 403 {
                return Err(anyhow!(
                    "Access denied. You may not have an active GitHub Copilot subscription."
                ));
            }

            return Err(anyhow!(
                "Failed to get Copilot token ({}): {}",
                status,
                text
            ));
        }

        response
            .json::<CopilotTokenResponse>()
            .await
            .context("Failed to parse Copilot token response")
    }

    /// Complete OAuth flow: from device code to Copilot session token
    pub async fn complete_flow(&self) -> Result<(String, CopilotTokenResponse)> {
        // Step 1: Get device code
        let device_code = self.request_device_code().await?;

        println!("\n🔑 GitHub Copilot Authentication");
        println!("═══════════════════════════════════════");
        println!("\nPlease visit: {}", device_code.verification_uri);
        println!("\nEnter code: {}\n", device_code.user_code);
        println!("Waiting for authorization...\n");

        // Step 2: Poll for OAuth token
        let github_token = self
            .poll_for_token(
                &device_code.device_code,
                device_code.interval,
                device_code.expires_in,
            )
            .await?;

        println!("✓ GitHub authorization successful!");
        println!("✓ Exchanging for Copilot token...\n");

        // Step 3: Get Copilot session token
        let copilot_token = self.get_copilot_token(&github_token).await?;

        Ok((github_token, copilot_token))
    }
}

impl Default for DeviceFlowAuth {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_id_is_set() {
        assert!(!CLIENT_ID.is_empty());
        assert_eq!(CLIENT_ID, "Iv1.b507a08c87ecfe98");
    }

    #[test]
    fn test_scope_is_correct() {
        assert_eq!(SCOPE, "read:user");
    }
}
