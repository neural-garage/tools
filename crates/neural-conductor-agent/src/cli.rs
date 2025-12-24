//! CLI commands for GitHub Copilot integration

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::copilot::{CopilotProvider, DeviceFlowAuth, StoredAuth, TokenStorage};

#[derive(Parser, Debug)]
#[command(name = "copilot")]
#[command(about = "GitHub Copilot integration commands")]
pub struct CopilotCli {
    #[command(subcommand)]
    pub command: CopilotCommand,
}

#[derive(Subcommand, Debug)]
pub enum CopilotCommand {
    /// Authenticate with GitHub Copilot
    Login,

    /// Show authentication status
    Status,

    /// Test Copilot API connection
    Test {
        /// Model to use for testing
        #[arg(short, long, default_value = "gpt-4o")]
        model: String,

        /// Test message to send
        #[arg(short, long, default_value = "Say hello!")]
        message: String,
    },

    /// Logout and clear stored credentials
    Logout,
}

impl CopilotCli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            CopilotCommand::Login => Self::login().await,
            CopilotCommand::Status => Self::status().await,
            CopilotCommand::Test { model, message } => Self::test(&model, &message).await,
            CopilotCommand::Logout => Self::logout().await,
        }
    }

    async fn login() -> Result<()> {
        println!("🚀 Starting GitHub Copilot authentication...\n");

        let auth = DeviceFlowAuth::new();
        let (github_token, copilot_token) = auth.complete_flow().await?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let stored = StoredAuth {
            github_token,
            copilot_token: copilot_token.token,
            expires_at: copilot_token.expires_at,
            refresh_in: copilot_token.refresh_in,
            updated_at: now,
        };

        let storage = TokenStorage::new()?;
        storage.save(&stored)?;

        println!("✅ Authentication successful!");
        println!("📁 Credentials saved to: {}", storage.path().display());
        println!("⏰ Session expires at: {}", copilot_token.expires_at);
        println!("\nYou can now use GitHub Copilot models in Neural Conductor! 🎉\n");

        Ok(())
    }

    async fn status() -> Result<()> {
        let storage = TokenStorage::new()?;

        if !storage.exists() {
            println!("❌ Not authenticated");
            println!("\nRun 'neural-conductor-agent copilot login' to authenticate.\n");
            return Ok(());
        }

        let stored = storage.load()?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let is_expired = now >= stored.expires_at;
        let time_until_expiry = if is_expired {
            "expired".to_string()
        } else {
            let seconds = stored.expires_at - now;
            if seconds < 3600 {
                format!("{} minutes", seconds / 60)
            } else if seconds < 86400 {
                format!("{} hours", seconds / 3600)
            } else {
                format!("{} days", seconds / 86400)
            }
        };

        println!("GitHub Copilot Status");
        println!("═══════════════════════════════════════");
        println!("✅ Authenticated");
        println!("📁 Config: {}", storage.path().display());
        println!("🔑 GitHub Token: {}...", &stored.github_token[..12]);
        println!(
            "⏰ Session expires: {} ({})",
            stored.expires_at, time_until_expiry
        );

        if is_expired {
            println!("\n⚠️  Session token is expired.");
            println!("It will be automatically refreshed on next API call.\n");
        } else {
            println!("\n✅ Session is valid\n");
        }

        Ok(())
    }

    async fn test(model: &str, message: &str) -> Result<()> {
        println!("🧪 Testing Copilot API connection...\n");
        println!("Model: {}", model);
        println!("Message: {}\n", message);

        let mut provider = CopilotProvider::from_storage()?;

        println!("📡 Sending request...");

        let request = crate::copilot::provider::ChatRequest {
            model: model.to_string(),
            messages: vec![crate::copilot::provider::ChatMessage {
                role: "user".to_string(),
                content: message.to_string(),
            }],
            temperature: Some(0.7),
            stream: Some(false),
            n: Some(1),
            prompt_cache_key: None,
        };

        let response = provider.chat_completion(request).await?;

        println!("✅ Response received!\n");
        println!("═══════════════════════════════════════");
        println!("Model: {}", response.model);
        println!("ID: {}", response.id);

        if let Some(choice) = response.choices.first() {
            println!("\nAssistant: {}", choice.message.content);
        }

        if let Some(usage) = response.usage {
            println!("\n📊 Token Usage:");
            println!("  Prompt: {}", usage.prompt_tokens);
            println!("  Completion: {}", usage.completion_tokens);
            println!("  Total: {}", usage.total_tokens);
            if let Some(cached) = usage.cached_tokens {
                println!("  Cached: {}", cached);
            }
        }

        println!("\n✅ Test completed successfully! 🎉\n");

        Ok(())
    }

    async fn logout() -> Result<()> {
        let storage = TokenStorage::new()?;

        if !storage.exists() {
            println!("ℹ️  No stored credentials found.\n");
            return Ok(());
        }

        storage.delete()?;
        println!("✅ Logged out successfully");
        println!("📁 Removed: {}\n", storage.path().display());

        Ok(())
    }
}
