# GitHub Copilot Integration for Neural Conductor Agent

This module integrates GitHub Copilot into the Neural Conductor agent, allowing it to use Copilot's LLM models for code analysis and generation.

## Features

- **OAuth Device Flow Authentication**: Secure authentication using GitHub's device flow
- **Automatic Token Refresh**: Session tokens are automatically refreshed when expired
- **Secure Token Storage**: Credentials stored with proper file permissions (0600)
- **Multiple Model Support**: Access to GPT-4o, Claude Sonnet 4.5, and other Copilot models
- **VSCode Compatibility**: Mimics VSCode extension headers for API compatibility
- **Enterprise Support**: Works with both github.com and GitHub Enterprise

## CLI Commands

### Login

Authenticate with GitHub Copilot:

```bash
neural-conductor-agent copilot login
```

This will:
1. Generate a device code
2. Display a URL and code for you to enter in your browser
3. Poll for authorization completion
4. Exchange for a Copilot session token
5. Save credentials to `~/.config/neural-conductor/copilot-auth.json`

### Status

Check authentication status:

```bash
neural-conductor-agent copilot status
```

Shows:
- Whether you're authenticated
- Token expiration time
- Credential file location

### Test

Test the Copilot API connection:

```bash
neural-conductor-agent copilot test
neural-conductor-agent copilot test --model claude-sonnet-4.5 --message "Hello!"
```

Sends a test message to the specified model and displays the response.

### Logout

Clear stored credentials:

```bash
neural-conductor-agent copilot logout
```

## Architecture

### Module Structure

```
src/copilot/
├── mod.rs        # Module exports
├── auth.rs       # OAuth device flow implementation
├── storage.rs    # Secure token storage
└── provider.rs   # Copilot API client
```

### Authentication Flow

1. **Device Code Request**: Request a device code from GitHub
2. **User Authorization**: User visits URL and enters code in browser
3. **Token Polling**: CLI polls for access token until user completes auth
4. **Token Exchange**: Exchange GitHub OAuth token for Copilot session token
5. **Secure Storage**: Save tokens with restricted permissions

### Token Management

Two types of tokens are used:

1. **GitHub OAuth Token** (`ghu_...`):
   - Long-lived refresh token
   - Valid for ~60 days
   - Used to obtain Copilot session tokens

2. **Copilot Session Token**:
   - Short-lived (24 hours)
   - Contains features, endpoints, and SKU information
   - Automatically refreshed when expired

### API Integration

The provider mimics VSCode extension behavior:

```rust
// Required headers for Copilot API compatibility
.header("User-Agent", "GitHubCopilotChat/0.32.4")
.header("Editor-Version", "vscode/1.105.1")
.header("Editor-Plugin-Version", "copilot-chat/0.32.4")
.header("Copilot-Integration-Id", "vscode-chat")
```

## Usage in Code

### Basic Usage

```rust
use neural_conductor_agent::copilot::{CopilotProvider, ChatRequest, ChatMessage};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load provider from stored credentials
    let mut provider = CopilotProvider::from_storage()?;

    // Create a chat request
    let request = ChatRequest {
        model: "gpt-4o".to_string(),
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Explain this code...".to_string(),
            },
        ],
        temperature: Some(0.7),
        stream: Some(false),
        n: Some(1),
        prompt_cache_key: None,
    };

    // Send request (auto-refreshes token if needed)
    let response = provider.chat_completion(request).await?;

    // Process response
    if let Some(choice) = response.choices.first() {
        println!("Assistant: {}", choice.message.content);
    }

    Ok(())
}
```

### Manual Authentication

```rust
use neural_conductor_agent::copilot::{DeviceFlowAuth, TokenStorage, StoredAuth};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Complete OAuth flow
    let auth = DeviceFlowAuth::new();
    let (github_token, copilot_token) = auth.complete_flow().await?;

    // Save credentials
    let stored = StoredAuth {
        github_token,
        copilot_token: copilot_token.token,
        expires_at: copilot_token.expires_at,
        refresh_in: copilot_token.refresh_in,
        updated_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
    };

    TokenStorage::new()?.save(&stored)?;

    Ok(())
}
```

## Available Models

### Included (Free on paid plans - 0x multiplier)
- `gpt-4o`
- `gpt-4.1`
- `gpt-5-mini`
- `raptor-mini`

### Budget (< 1x multiplier)
- `grok-code-fast-1` (0.25x)
- `claude-haiku-4.5` (0.33x)
- `gpt-5.1-codex-mini` (0.33x)

### Standard (1x multiplier)
- `claude-sonnet-4`
- `claude-sonnet-4.5`
- `gemini-2.5-pro`
- `gemini-3-pro`
- `gpt-5`
- `gpt-5.1`
- `gpt-5.1-codex`
- `gpt-5.1-codex-max`
- `gpt-5.2`

### Premium (> 1x multiplier)
- `claude-opus-4.5` (3x)
- `claude-opus-4.1` (10x)

## Billing Considerations

**Important**: Each API call counts as a separate premium request, even within the same conversation.

- GitHub Copilot Pro: 300 premium requests/month
- Each user message = 1 premium request × model multiplier
- Prompt caching reduces token costs but NOT request counts

Example:
- 5 messages with `gpt-4o`: 0 premium requests used
- 5 messages with `claude-sonnet-4.5`: 5 premium requests used
- 5 messages with `claude-opus-4.1`: 50 premium requests used

**Recommendation**: Use included models (`gpt-4o`) for most tasks to conserve premium requests.

## Security

- Credentials stored with 0600 permissions (owner read/write only)
- Storage directory has 0700 permissions (owner full access only)
- Warning displayed if insecure permissions detected
- Tokens never logged or displayed in plain text

## Implementation Details

### OAuth Client Credentials

```rust
const CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";
const SCOPE: &str = "read:user";
```

These are public client credentials used by the VSCode Copilot extension.

### API Endpoints

- Device Code: `https://github.com/login/device/code`
- Access Token: `https://github.com/login/oauth/access_token`
- Copilot Token: `https://api.github.com/copilot_internal/v2/token`
- Chat Completions: `https://api.githubcopilot.com/chat/completions`

### Error Handling

All public APIs return `anyhow::Result<T>` for comprehensive error handling:

```rust
if let Err(e) = provider.chat_completion(request).await {
    eprintln!("Error: {}", e);
    // e contains full error chain with context
}
```

## Future Enhancements

- [ ] Streaming response support
- [ ] Conversation management with prompt caching
- [ ] Load `.github/copilot-instructions.md` files
- [ ] Usage tracking and analytics
- [ ] Model selection based on task complexity
- [ ] Rate limiting and retry logic
- [ ] GitHub Enterprise URL configuration

## References

- [GitHub Copilot Integration Guide](../../../GITHUB_COPILOT_INTEGRATION_GUIDE.md)
- OpenCode implementation: `/Users/paolo/dev/opencode`
- Neural Conductor Server: `/Users/paolo/dev/conductor`

## License

Part of the Neural Garage Tools project.
