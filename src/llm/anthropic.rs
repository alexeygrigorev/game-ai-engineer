//! Anthropic/Z.ai LLM Provider
//!
//! Implements the LlmProvider trait for Anthropic-compatible APIs.
//! Currently configured for Z.ai (https://api.z.ai/api/anthropic).
//!
//! # Configuration
//! Set in .env:
//! - `ANTHROPIC_API_KEY`: Your API key
//! - `ANTHROPIC_BASE_URL`: API endpoint (e.g., https://api.z.ai/api/anthropic)
//!
//! # Example
//! ```rust
//! use crate::llm::{LlmProvider, LlmMessage};
//! use crate::llm::anthropic::AnthropicProvider;
//!
//! let provider = AnthropicProvider::new("glm-4.7")?;
//! let response = provider.complete("You are helpful", vec![LlmMessage::user("Hello")]).await?;
//! ```

use std::future::Future;
use std::pin::Pin;
use std::env;
use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::provider::LlmMessage;

/// Anthropic/Z.ai API client
#[derive(Clone)]
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider
    ///
    /// Reads credentials from environment variables:
    /// - `ANTHROPIC_API_KEY`: Required
    /// - `ANTHROPIC_BASE_URL`: Required
    ///
    /// # Errors
    /// Returns error if environment variables are not set
    pub fn new(model: &str) -> Result<Self> {
        dotenvy::dotenv().ok();
        
        let api_key = env::var("ANTHROPIC_API_KEY")
            .context("ANTHROPIC_API_KEY not found in environment")?;
        let base_url = env::var("ANTHROPIC_BASE_URL")
            .context("ANTHROPIC_BASE_URL not found in environment")?;
        
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
            model: model.to_string(),
        })
    }
    
    /// Create provider with explicit credentials (for testing)
    #[cfg(test)]
    pub fn with_credentials(api_key: String, base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
            model,
        }
    }
}

impl super::provider::LlmProvider for AnthropicProvider {
    fn name(&self) -> &str { 
        "anthropic" 
    }
    
    fn complete<'a>(&'a self, system: &'a str, messages: Vec<LlmMessage>) 
        -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> 
    {
        Box::pin(async move {
            let anthropic_messages: Vec<AnthropicMessage> = messages
                .into_iter()
                .map(|m| AnthropicMessage {
                    role: m.role,
                    content: vec![ContentBlock {
                        content_type: "text".to_string(),
                        text: m.content,
                    }],
                })
                .collect();

            let body = serde_json::json!({
                "model": self.model,
                "max_tokens": 1024,
                "system": system,
                "messages": anthropic_messages,
            });

            let response = self.client
                .post(format!("{}/v1/messages", self.base_url))
                .header("x-api-key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .context("Failed to send request to Anthropic API")?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("API error ({}): {}", status, body);
            }

            let response_text = response.text().await?;
            let api_response: ApiResponse = serde_json::from_str(&response_text)
                .with_context(|| format!("Failed to parse API response: {}", response_text))?;

            let text = api_response.content
                .into_iter()
                .find(|c| c.content_type == "text")
                .and_then(|c| c.text)
                .ok_or_else(|| anyhow::anyhow!("No text content in response"))?;

            Ok(text)
        })
    }
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<ContentBlock>,
}

#[derive(Debug, Serialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    content: Vec<ApiContent>,
}

#[derive(Debug, Deserialize)]
struct ApiContent {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(default)]
    text: Option<String>,
}
