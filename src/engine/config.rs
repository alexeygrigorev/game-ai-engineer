//! Game Configuration
//!
//! Controls which engine (rule/llm) each activity uses.
//! Embedded in binary at compile time, loaded on startup.
//!
//! # Config File Structure (game_config.toml)
//! ```toml
//! [llm]
//! provider = "anthropic"
//! model = "glm-4.7"
//!
//! [npc]
//! default_engine = "rule"
//!
//! [npc.classes.recruiter]
//! engine = "llm"
//! persona = "You are a tech recruiter..."
//!
//! [interview]
//! engine = "llm"
//! ```

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

use super::traits::EngineType;

/// LLM configuration
#[derive(Debug, Clone, Deserialize)]
pub struct LlmConfig {
    /// Provider name: "anthropic" or "mock"
    pub provider: String,
    /// Model identifier
    pub model: String,
}

/// NPC class configuration
#[derive(Debug, Clone, Deserialize)]
pub struct NpcClassConfig {
    /// Engine type for this class
    #[serde(default)]
    pub engine: Option<String>,
    /// LLM persona template
    pub persona: Option<String>,
    /// Fallback dialog lines for rule engine
    #[serde(default)]
    pub fallback_dialog: Vec<String>,
}

/// NPC configuration
#[derive(Debug, Clone, Deserialize)]
pub struct NpcConfig {
    /// Default engine for all NPCs
    #[serde(default)]
    pub default_engine: String,
    /// Per-class configuration
    #[serde(default)]
    pub classes: HashMap<String, NpcClassConfig>,
}

/// Interview configuration
#[derive(Debug, Clone, Deserialize)]
pub struct InterviewConfig {
    /// Engine type for interviews
    #[serde(default)]
    pub engine: String,
}

/// Root game configuration
#[derive(Debug, Clone, Deserialize)]
pub struct GameConfig {
    pub llm: LlmConfig,
    #[serde(default)]
    pub npc: NpcConfig,
    #[serde(default)]
    pub interview: InterviewConfig,
}

impl Default for NpcConfig {
    fn default() -> Self {
        Self {
            default_engine: "rule".to_string(),
            classes: HashMap::new(),
        }
    }
}

impl Default for InterviewConfig {
    fn default() -> Self {
        Self {
            engine: "rule".to_string(),
        }
    }
}

impl GameConfig {
    /// Load embedded config from game_config.toml
    ///
    /// The config file is embedded in the binary at compile time.
    pub fn load() -> Result<Self> {
        const CONFIG: &str = include_str!("../config/game_config.toml");
        toml::from_str(CONFIG).context("Failed to parse game_config.toml")
    }

    /// Get the engine type for an NPC class
    ///
    /// Falls back to default_engine if class not configured
    pub fn get_npc_engine(&self, class_name: &str) -> EngineType {
        if let Some(class) = self.npc.classes.get(class_name) {
            if let Some(engine) = &class.engine {
                return engine.parse().unwrap_or(EngineType::Rule);
            }
        }
        self.npc.default_engine.parse().unwrap_or(EngineType::Rule)
    }

    /// Get persona for an NPC class
    pub fn get_npc_persona(&self, class_name: &str) -> Option<&str> {
        self.npc
            .classes
            .get(class_name)
            .and_then(|c| c.persona.as_deref())
    }

    /// Get fallback dialog for an NPC class
    pub fn get_npc_fallback_dialog(&self, class_name: &str) -> Option<&Vec<String>> {
        self.npc.classes.get(class_name).map(|c| &c.fallback_dialog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = GameConfig::load().expect("Failed to load config");
        assert!(!config.llm.provider.is_empty());
    }

    #[test]
    fn test_get_npc_engine_default() {
        let config = GameConfig::load().unwrap();
        let engine = config.get_npc_engine("unknown_class");
        assert_eq!(engine, EngineType::Rule);
    }
}
