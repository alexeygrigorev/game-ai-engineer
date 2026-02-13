//! Activity Engine Traits
//!
//! Defines the common interface for all game activities that can be
//! powered by rules or LLM.
//!
//! # Engine Types
//! - `Rule`: Use hardcoded logic (fast, predictable, no API cost)
//! - `Llm`: Use LLM for dynamic responses
//! - `Hybrid`: Try LLM first, fallback to rule on error

use anyhow::Result;
use std::future::Future;

/// How an activity generates content
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineType {
    /// Hardcoded logic (fast, no API cost)
    Rule,
    /// LLM-powered (dynamic responses)
    Llm,
    /// Try LLM, fallback to rule on error
    Hybrid,
}

impl Default for EngineType {
    fn default() -> Self {
        Self::Rule
    }
}

impl std::fmt::Display for EngineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rule => write!(f, "rule"),
            Self::Llm => write!(f, "llm"),
            Self::Hybrid => write!(f, "hybrid"),
        }
    }
}

impl std::str::FromStr for EngineType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rule" => Ok(Self::Rule),
            "llm" => Ok(Self::Llm),
            "hybrid" => Ok(Self::Hybrid),
            _ => Err(format!("Unknown engine type: {}", s)),
        }
    }
}

/// Trait for game activities that can be LLM-powered
///
/// Each activity (NPC dialog, interviews, etc.) implements this trait
/// and decides how to handle Rule vs Llm modes internally.
///
/// # Example
/// ```rust
/// struct MyActivity {
///     engine_type: EngineType,
///     provider: Arc<dyn LlmProvider>,
/// }
///
/// impl ActivityEngine for MyActivity {
///     type Input = String;
///     type Output = String;
///
///     async fn execute(&self, input: Self::Input, context: &GameContext) -> Result<Self::Output> {
///         match self.engine_type {
///             EngineType::Rule => self.rule_logic(&input),
///             EngineType::Llm => self.llm_logic(&input, context).await,
///             EngineType::Hybrid => {
///                 self.llm_logic(&input, context).await
///                     .or_else(|_| self.rule_logic(&input))
///             }
///         }
///     }
///
///     fn engine_type(&self) -> EngineType {
///         self.engine_type
///     }
/// }
/// ```
pub trait ActivityEngine: Send + Sync {
    /// Input type for this activity
    type Input: Send;
    /// Output type for this activity
    type Output: Send;

    /// Execute the activity with given input and game context
    fn execute(
        &self,
        input: Self::Input,
        context: &crate::engine::context::GameContext,
    ) -> impl Future<Output = Result<Self::Output>> + Send;

    /// Which engine type this activity is configured to use
    fn engine_type(&self) -> EngineType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_engine_type_parse() {
        assert_eq!(EngineType::from_str("rule").unwrap(), EngineType::Rule);
        assert_eq!(EngineType::from_str("LLM").unwrap(), EngineType::Llm);
        assert_eq!(EngineType::from_str("Hybrid").unwrap(), EngineType::Hybrid);
        assert!(EngineType::from_str("invalid").is_err());
    }

    #[test]
    fn test_engine_type_display() {
        assert_eq!(EngineType::Rule.to_string(), "rule");
        assert_eq!(EngineType::Llm.to_string(), "llm");
        assert_eq!(EngineType::Hybrid.to_string(), "hybrid");
    }
}
