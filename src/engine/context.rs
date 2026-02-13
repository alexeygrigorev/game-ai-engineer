//! Game Context for LLM Prompts
//!
//! This data is injected into LLM system prompts so NPCs and activities
//! can respond appropriately to the player's current state.
//!
//! # What's Included
//! - Player identity (name)
//! - Skills (top 5 by level)
//! - Employment status
//! - Current day in game
//!
//! # What's NOT Included (for now)
//! - Inventory (not relevant)
//! - Quest progress (not implemented)
//! - Location (NPC already knows their context)

use std::collections::HashMap;

/// Information about a single skill
#[derive(Debug, Clone)]
pub struct SkillInfo {
    /// Skill name (e.g., "Python", "PyTorch")
    pub name: String,
    /// Proficiency level (e.g., "Beginner", "Intermediate", "Expert")
    pub proficiency: String,
}

/// Game state passed to LLM for context-aware responses
///
/// This struct is passed to activity engines so they can include
/// relevant player info in LLM prompts.
#[derive(Debug, Clone)]
pub struct GameContext {
    /// Player's display name
    pub player_name: String,
    /// Top 5 skills by level (name, proficiency)
    pub top_skills: Vec<SkillInfo>,
    /// Whether the player is currently employed
    pub employed: bool,
    /// Current job title if employed
    pub current_job: Option<String>,
    /// Current day number in game
    pub day: u32,
}

impl GameContext {
    /// Create an empty context for testing
    pub fn empty() -> Self {
        Self {
            player_name: "Player".to_string(),
            top_skills: vec![],
            employed: false,
            current_job: None,
            day: 1,
        }
    }

    /// Create context from game state
    pub fn from_game_state(
        player_name: &str,
        skills: &HashMap<String, crate::player::PlayerSkill>,
        employed: bool,
        current_job: Option<&str>,
        day: u32,
    ) -> Self {
        let mut skill_list: Vec<_> = skills
            .iter()
            .map(|(name, skill)| {
                let proficiency = skill.proficiency.as_str().to_string();
                let level = skill.proficiency as u8;
                (name.clone(), proficiency, level)
            })
            .collect();

        // Sort by level descending, take top 5
        skill_list.sort_by(|a, b| b.2.cmp(&a.2));
        skill_list.truncate(5);

        let top_skills = skill_list
            .into_iter()
            .map(|(name, proficiency, _)| SkillInfo { name, proficiency })
            .collect();

        Self {
            player_name: player_name.to_string(),
            top_skills,
            employed,
            current_job: current_job.map(|s| s.to_string()),
            day,
        }
    }

    /// Format for inclusion in LLM system prompt
    ///
    /// Creates a readable section describing the player's current state.
    pub fn to_prompt_section(&self) -> String {
        let skills_str = if self.top_skills.is_empty() {
            "None yet".to_string()
        } else {
            self.top_skills
                .iter()
                .map(|s| format!("{} ({})", s.name, s.proficiency))
                .collect::<Vec<_>>()
                .join(", ")
        };

        let employment_str = match (&self.employed, &self.current_job) {
            (true, Some(job)) => format!("Yes - {}", job),
            (true, None) => "Yes".to_string(),
            (false, _) => "No, looking for opportunities".to_string(),
        };

        format!(
            "PLAYER INFO:\n\
             - Name: {}\n\
             - Skills: {}\n\
             - Employed: {}\n\
             - Current Day: {}",
            self.player_name, skills_str, employment_str, self.day,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_context() {
        let ctx = GameContext::empty();
        assert_eq!(ctx.player_name, "Player");
        assert!(ctx.top_skills.is_empty());
        assert!(!ctx.employed);
    }

    #[test]
    fn test_to_prompt_section() {
        let ctx = GameContext {
            player_name: "Alice".to_string(),
            top_skills: vec![
                SkillInfo {
                    name: "Python".to_string(),
                    proficiency: "Expert".to_string(),
                },
                SkillInfo {
                    name: "SQL".to_string(),
                    proficiency: "Intermediate".to_string(),
                },
            ],
            employed: false,
            current_job: None,
            day: 5,
        };

        let prompt = ctx.to_prompt_section();
        assert!(prompt.contains("Alice"));
        assert!(prompt.contains("Python (Expert)"));
        assert!(prompt.contains("SQL (Intermediate)"));
        assert!(prompt.contains("looking for opportunities"));
        assert!(prompt.contains("Day: 5"));
    }
}
