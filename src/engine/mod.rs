//! Game Engine Module
//!
//! Provides activity engines for game features that can be powered by
//! rules (hardcoded) or LLM (dynamic responses).
//!
//! # Architecture
//! ```text
//! ┌──────────────┐
//! │  Game Loop   │
//! └──────┬───────┘
//!        │ uses
//!        ▼
//! ┌──────────────┐     ┌──────────────┐
//! │  NpcEngine   │     │ InterviewEng │ ...
//! └──────┬───────┘     └──────┬───────┘
//!        │                    │
//!        └────────┬───────────┘
//!                 │
//!                 ▼
//!        ┌────────────────┐
//!        │   LlmProvider  │ (shared)
//!        └────────────────┘
//! ```
//!
//! # Engine Types
//! - `Rule`: Hardcoded logic (fast, predictable, no API cost)
//! - `Llm`: LLM-powered (dynamic, uses API)
//! - `Hybrid`: Try LLM first, fallback to rule on error

pub mod traits;
pub mod config;
pub mod context;
pub mod cache;
pub mod npc;

pub use traits::{ActivityEngine, EngineType};
pub use config::GameConfig;
pub use context::GameContext;
pub use cache::ResponseCache;
pub use npc::NpcEngine;
