//! Mock LLM Provider for Testing
//!
//! A simple mock provider that returns predefined responses.
//! Useful for unit testing without making real API calls.
//!
//! # Example
//! ```rust
//! use crate::llm::mock::MockProvider;
//! use crate::llm::{LlmProvider, LlmMessage};
//!
//! let mock = MockProvider::new("Hello back!");
//! let response = mock.complete("system", vec![LlmMessage::user("Hello")]).await?;
//! assert_eq!(response, "Hello back!");
//! ```

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use anyhow::Result;

use super::provider::LlmMessage;

/// Mock provider that returns predefined responses
///
/// Thread-safe for use in async tests.
#[derive(Clone)]
pub struct MockProvider {
    /// Name of this mock provider
    name: String,
    /// Response to return for any completion
    response: Arc<Mutex<String>>,
    /// Optional: track all requests made (for assertions)
    requests: Arc<Mutex<Vec<(String, Vec<LlmMessage>)>>>,
}

impl MockProvider {
    /// Create a mock that returns a fixed response
    pub fn new(response: impl Into<String>) -> Self {
        Self {
            name: "mock".to_string(),
            response: Arc::new(Mutex::new(response.into())),
            requests: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Create a mock with a custom name
    pub fn with_name(name: impl Into<String>, response: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            response: Arc::new(Mutex::new(response.into())),
            requests: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Update the response (for testing different scenarios)
    pub fn set_response(&self, response: impl Into<String>) {
        *self.response.lock().unwrap() = response.into();
    }
    
    /// Get all requests made to this mock (for assertions)
    pub fn get_requests(&self) -> Vec<(String, Vec<LlmMessage>)> {
        self.requests.lock().unwrap().clone()
    }
    
    /// Clear request history
    pub fn clear_requests(&self) {
        self.requests.lock().unwrap().clear();
    }
}

impl super::provider::LlmProvider for MockProvider {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn complete<'a>(&'a self, system: &'a str, messages: Vec<LlmMessage>) 
        -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> 
    {
        Box::pin(async move {
            // Track the request
            self.requests.lock().unwrap().push((system.to_string(), messages));
            
            // Return the predefined response
            Ok(self.response.lock().unwrap().clone())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::provider::LlmProvider;
    
    #[tokio::test]
    async fn test_mock_returns_fixed_response() {
        let mock = MockProvider::new("Test response");
        let result = mock.complete("system", vec![LlmMessage::user("hello")]).await.unwrap();
        assert_eq!(result, "Test response");
    }
    
    #[tokio::test]
    async fn test_mock_tracks_requests() {
        let mock = MockProvider::new("response");
        mock.complete("my system", vec![LlmMessage::user("hello")]).await.unwrap();
        
        let requests = mock.get_requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].0, "my system");
        assert_eq!(requests[0].1.len(), 1);
    }
    
    #[tokio::test]
    async fn test_mock_can_update_response() {
        let mock = MockProvider::new("first");
        assert_eq!(mock.complete("", vec![LlmMessage::user("test")]).await.unwrap(), "first");
        
        mock.set_response("second");
        assert_eq!(mock.complete("", vec![LlmMessage::user("test")]).await.unwrap(), "second");
    }
}
