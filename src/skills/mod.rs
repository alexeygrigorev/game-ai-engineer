//! Skills Module
//!
//! Defines skills, categories, and proficiency levels for the game.
//! Skills are loaded from config/skills.toml at compile time.

use serde::{Deserialize, Serialize};

/// Skill categories for organizing skills
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum SkillCategory {
    MlAlgorithms,
    Statistics,
    Programming,
    Databases,
    SoftSkills,
    DomainKnowledge,
}

/// Proficiency levels for skills
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Proficiency {
    None = 0,
    Basic = 1,
    Intermediate = 2,
    Advanced = 3,
    Expert = 4,
}

impl Default for Proficiency {
    fn default() -> Self {
        Proficiency::None
    }
}

impl Proficiency {
    pub fn next(&self) -> Option<Proficiency> {
        match self {
            Proficiency::None => Some(Proficiency::Basic),
            Proficiency::Basic => Some(Proficiency::Intermediate),
            Proficiency::Intermediate => Some(Proficiency::Advanced),
            Proficiency::Advanced => Some(Proficiency::Expert),
            Proficiency::Expert => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Proficiency::None => "None",
            Proficiency::Basic => "Basic",
            Proficiency::Intermediate => "Intermediate",
            Proficiency::Advanced => "Advanced",
            Proficiency::Expert => "Expert",
        }
    }
}

impl std::str::FromStr for Proficiency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Proficiency::None),
            "Basic" => Ok(Proficiency::Basic),
            "Intermediate" => Ok(Proficiency::Intermediate),
            "Advanced" => Ok(Proficiency::Advanced),
            "Expert" => Ok(Proficiency::Expert),
            _ => Err(format!("Unknown proficiency: {}", s)),
        }
    }
}

/// A skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub category: SkillCategory,
    pub description: String,
    pub difficulty: u8,
}

impl Skill {
    pub fn new(name: &str, category: SkillCategory, description: &str, difficulty: u8) -> Self {
        Self {
            name: name.to_string(),
            category,
            description: description.to_string(),
            difficulty,
        }
    }
}

/// Skills configuration loaded from TOML
#[derive(Debug, Clone, Deserialize)]
struct SkillsConfig {
    skills: Vec<Skill>,
}

/// Load all skills from config file
pub fn get_all_skills() -> Vec<Skill> {
    const CONFIG: &str = include_str!("../config/skills.toml");
    let config: SkillsConfig = toml::from_str(CONFIG).expect("Failed to parse skills.toml");
    config.skills
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proficiency_ordering() {
        assert!(Proficiency::Basic > Proficiency::None);
        assert!(Proficiency::Intermediate > Proficiency::Basic);
        assert!(Proficiency::Advanced > Proficiency::Intermediate);
        assert!(Proficiency::Expert > Proficiency::Advanced);
    }

    #[test]
    fn test_proficiency_next() {
        assert_eq!(Proficiency::None.next(), Some(Proficiency::Basic));
        assert_eq!(Proficiency::Basic.next(), Some(Proficiency::Intermediate));
        assert_eq!(
            Proficiency::Intermediate.next(),
            Some(Proficiency::Advanced)
        );
        assert_eq!(Proficiency::Advanced.next(), Some(Proficiency::Expert));
        assert_eq!(Proficiency::Expert.next(), None);
    }

    #[test]
    fn test_proficiency_as_str() {
        assert_eq!(Proficiency::None.as_str(), "None");
        assert_eq!(Proficiency::Basic.as_str(), "Basic");
        assert_eq!(Proficiency::Expert.as_str(), "Expert");
    }

    #[test]
    fn test_get_all_skills() {
        let skills = get_all_skills();
        assert!(skills.len() > 0);

        let python = skills.iter().find(|s| s.name == "Python");
        assert!(python.is_some());
        assert_eq!(python.unwrap().category, SkillCategory::Programming);
        assert_eq!(python.unwrap().difficulty, 1);
    }

    #[test]
    fn test_skill_categories() {
        let skills = get_all_skills();

        let ml_skills: Vec<_> = skills
            .iter()
            .filter(|s| s.category == SkillCategory::MlAlgorithms)
            .collect();
        assert!(ml_skills.len() >= 4);

        let programming_skills: Vec<_> = skills
            .iter()
            .filter(|s| s.category == SkillCategory::Programming)
            .collect();
        assert!(programming_skills.len() >= 2);
    }

    #[test]
    fn test_skill_creation() {
        let skill = Skill::new("TestSkill", SkillCategory::Programming, "A test skill", 2);
        assert_eq!(skill.name, "TestSkill");
        assert_eq!(skill.category, SkillCategory::Programming);
        assert_eq!(skill.description, "A test skill");
        assert_eq!(skill.difficulty, 2);
    }
}
