//! LLM Module
//!
//! Provides LLM integration for game activities. Supports multiple providers
//! (Anthropic, OpenAI, etc.) through a common trait interface.
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
//! # Testing
//! - **Unit tests**: Use `MockProvider` (no API calls)
//! - **Integration tests**: Use `AnthropicProvider` with real API
//!
//! # Example
//! ```rust
//! use crate::llm::{LlmProvider, LlmMessage, create_provider, LlmConfig};
//!
//! let config = LlmConfig {
//!     provider: "anthropic".into(),
//!     model: "glm-4.7".into(),
//! };
//! let provider = create_provider(&config)?;
//! let response = provider.complete(
//!     "You are helpful",
//!     vec![LlmMessage::user("Hello")],
//! ).await?;
//! ```

pub mod provider;
pub mod anthropic;
pub mod mock;

pub use provider::{LlmProvider, LlmMessage, LlmConfig, Provider, create_provider};
pub use anthropic::AnthropicProvider;
pub use mock::MockProvider;

#[cfg(test)]
mod tests {
    use super::*;
    use provider::LlmProvider;
    
    #[test]
    fn test_message_constructors() {
        let user = LlmMessage::user("hello");
        assert_eq!(user.role, "user");
        assert_eq!(user.content, "hello");
        
        let assistant = LlmMessage::assistant("hi there");
        assert_eq!(assistant.role, "assistant");
        
        let system = LlmMessage::system("be helpful");
        assert_eq!(system.role, "system");
    }
    
    #[tokio::test]
    async fn test_mock_provider() {
        let mock = MockProvider::new("Test response");
        let result = mock.complete("system", vec![LlmMessage::user("test")]).await.unwrap();
        assert_eq!(result, "Test response");
    }
    
    #[tokio::test]
    async fn test_provider_enum() {
        let config = LlmConfig {
            provider: "mock".into(),
            model: "test".into(),
        };
        let provider = create_provider(&config).unwrap();
        let result = provider.complete("system", vec![LlmMessage::user("test")]).await.unwrap();
        assert_eq!(result, "Mock response");
    }
}
