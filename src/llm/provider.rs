//! LLM Provider Abstraction
//!
//! This module defines the trait for LLM providers, allowing the game
//! to switch between different providers (Anthropic, OpenAI, etc.)
//! without changing activity code.
//!
//! # Architecture
//! ```text
//! ┌─────────────────┐
//! │ Game Activities │ (NPC, Interview, etc.)
//! └────────┬────────┘
//!          │ uses
//!          ▼
//! ┌─────────────────┐
//! │    Provider     │ (enum)
//! └────────┬────────┘
//!          │ variants
//!     ┌────┼────┐
//!     ▼    ▼    ▼
//! Anthropic Mock OpenAI
//!           (test) (future)
//! ```
//!
//! # Adding a New Provider
//! 1. Implement `LlmProvider` trait
//! 2. Add provider variant to `Provider` enum
//! 3. Add configuration section to `game_config.toml`

use anyhow::{anyhow, Result};
use std::future::Future;
use std::pin::Pin;

/// Represents a single message in a conversation with an LLM
#[derive(Debug, Clone)]
pub struct LlmMessage {
    /// Role: "user", "assistant", or "system"
    pub role: String,
    /// Message content
    pub content: String,
}

impl LlmMessage {
    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: content.into(),
        }
    }

    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }
}

/// Trait for LLM providers (async methods return boxed futures for dyn compatibility)
///
/// Implement this trait to add support for new LLM backends.
pub trait LlmProvider: Send + Sync {
    /// Send a completion request to the LLM
    fn complete<'a>(
        &'a self,
        system: &'a str,
        messages: Vec<LlmMessage>,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;

    /// Human-readable name for logging and debugging
    fn name(&self) -> &str;
}

/// Provider enum for compile-time provider selection
///
/// This enum allows the game to work with different providers
/// without dynamic dispatch overhead in hot paths.
#[derive(Clone)]
pub enum Provider {
    /// Anthropic/Z.ai provider
    Anthropic(crate::llm::anthropic::AnthropicProvider),
    /// Mock provider for testing
    Mock(crate::llm::mock::MockProvider),
}

impl LlmProvider for Provider {
    fn name(&self) -> &str {
        match self {
            Self::Anthropic(p) => p.name(),
            Self::Mock(p) => p.name(),
        }
    }

    fn complete<'a>(
        &'a self,
        system: &'a str,
        messages: Vec<LlmMessage>,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        match self {
            Self::Anthropic(p) => p.complete(system, messages),
            Self::Mock(p) => p.complete(system, messages),
        }
    }
}

/// Configuration for creating an LLM provider
#[derive(Debug, Clone)]
pub struct LlmConfig {
    /// Provider name: "anthropic", "mock", etc.
    pub provider: String,
    /// Model identifier (provider-specific)
    pub model: String,
}

/// Create an LLM provider based on configuration
///
/// # Currently Supported Providers
/// - `"anthropic"`: Anthropic/Z.ai API
/// - `"mock"`: Mock provider for testing
///
/// # Errors
/// Returns an error if the provider name is unknown
pub fn create_provider(config: &LlmConfig) -> Result<Provider> {
    match config.provider.as_str() {
        "anthropic" => {
            let provider = crate::llm::anthropic::AnthropicProvider::new(&config.model)?;
            Ok(Provider::Anthropic(provider))
        }
        "mock" => {
            let provider = crate::llm::mock::MockProvider::new("Mock response");
            Ok(Provider::Mock(provider))
        }
        _ => Err(anyhow!(
            "Unknown LLM provider: {}. Supported: anthropic, mock",
            config.provider
        )),
    }
}
