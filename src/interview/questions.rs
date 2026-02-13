//! Interview Questions Module
//!
//! Loads interview questions from config/interview_questions.toml.
//! Questions are organized by skill name.

use serde::Deserialize;

/// A single interview question
#[derive(Debug, Clone, Deserialize)]
pub struct InterviewQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub correct_idx: usize,
}

/// Questions for a single skill
#[derive(Debug, Clone, Deserialize)]
struct SkillQuestions {
    name: String,
    questions: Vec<InterviewQuestion>,
}

/// Root config structure
#[derive(Debug, Clone, Deserialize)]
struct InterviewQuestionsConfig {
    skill: Vec<SkillQuestions>,
}

/// Interview question database
///
/// Stores all questions loaded from config, organized by skill name.
pub struct InterviewQuestionDb {
    questions_by_skill: std::collections::HashMap<String, Vec<InterviewQuestion>>,
    default_questions: Vec<InterviewQuestion>,
}

impl InterviewQuestionDb {
    /// Load questions from embedded config file
    pub fn load() -> Self {
        const CONFIG: &str = include_str!("../config/interview_questions.toml");
        let config: InterviewQuestionsConfig =
            toml::from_str(CONFIG).expect("Failed to parse interview_questions.toml");

        let mut questions_by_skill = std::collections::HashMap::new();
        let mut default_questions = Vec::new();

        for skill in config.skill {
            if skill.name == "default" {
                default_questions = skill.questions;
            } else {
                questions_by_skill.insert(skill.name, skill.questions);
            }
        }

        Self {
            questions_by_skill,
            default_questions,
        }
    }

    /// Get questions for a skill
    ///
    /// Returns questions for the skill, or default questions if not found.
    /// Skill names with spaces should be passed as-is (e.g., "LLM Fine-tuning").
    pub fn get_questions(&self, skill_name: &str) -> &[InterviewQuestion] {
        // Try exact match first
        if let Some(questions) = self.questions_by_skill.get(skill_name) {
            return questions;
        }

        // Try with spaces replaced by underscores (TOML key workaround)
        let normalized = skill_name.replace(' ', "_");
        if let Some(questions) = self.questions_by_skill.get(&normalized) {
            return questions;
        }

        // Fall back to default
        &self.default_questions
    }

    /// Get a random question for a skill
    pub fn get_random_question(&self, skill_name: &str) -> Option<&InterviewQuestion> {
        use rand::seq::SliceRandom;
        let questions = self.get_questions(skill_name);
        questions.choose(&mut rand::thread_rng())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_db() {
        let db = InterviewQuestionDb::load();
        assert!(!db.questions_by_skill.is_empty());
    }

    #[test]
    fn test_get_python_questions() {
        let db = InterviewQuestionDb::load();
        let questions = db.get_questions("Python");
        assert!(!questions.is_empty());
    }

    #[test]
    fn test_get_default_questions() {
        let db = InterviewQuestionDb::load();
        let questions = db.get_questions("UnknownSkill");
        assert!(!questions.is_empty());
    }

    #[test]
    fn test_skill_with_space() {
        let db = InterviewQuestionDb::load();
        let questions = db.get_questions("LLM Fine-tuning");
        assert!(!questions.is_empty());
    }
}
