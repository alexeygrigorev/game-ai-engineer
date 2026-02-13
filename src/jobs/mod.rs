use serde::{Deserialize, Serialize};

use crate::skills::Proficiency;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRequirement {
    pub skill_name: String,
    pub min_proficiency: Proficiency,
    pub mandatory: bool,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub company: String,
    pub salary_min: u32,
    pub salary_max: u32,
    pub requirements: Vec<SkillRequirement>,
    pub min_experience_days: u32,
    pub description: String,
    pub difficulty: u8,
}

impl Job {
    pub fn calculate_match(&self, player_skills: &std::collections::HashMap<String, crate::player::PlayerSkill>) -> f32 {
        let mut total_weight = 0.0;
        let mut matched_weight = 0.0;

        for req in &self.requirements {
            total_weight += req.weight;
            
            let proficiency = player_skills
                .get(&req.skill_name)
                .map(|s| s.proficiency)
                .unwrap_or(Proficiency::None);

            if proficiency >= req.min_proficiency {
                matched_weight += req.weight;
            } else if proficiency != Proficiency::None {
                let ratio = (proficiency as i32 as f32) / (req.min_proficiency as i32 as f32);
                matched_weight += req.weight * ratio * 0.5;
            }
        }

        if total_weight > 0.0 {
            matched_weight / total_weight
        } else {
            0.0
        }
    }

    pub fn display_salary(&self) -> String {
        format!("${} - ${}/year", self.salary_min, self.salary_max)
    }
}

#[derive(Debug, Clone)]
pub struct Company {
    pub name: String,
    pub description: String,
    pub tier: CompanyTier,
    pub open_positions: Vec<Job>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompanyTier {
    Startup,
    MidSize,
    BigTech,
    Faang,
}

impl CompanyTier {
    pub fn salary_multiplier(&self) -> f32 {
        match self {
            CompanyTier::Startup => 0.8,
            CompanyTier::MidSize => 1.0,
            CompanyTier::BigTech => 1.5,
            CompanyTier::Faang => 2.0,
        }
    }

    pub fn difficulty_modifier(&self) -> u8 {
        match self {
            CompanyTier::Startup => 0,
            CompanyTier::MidSize => 1,
            CompanyTier::BigTech => 2,
            CompanyTier::Faang => 3,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CompanyTier::Startup => "Startup",
            CompanyTier::MidSize => "Mid-Size",
            CompanyTier::BigTech => "Big Tech",
            CompanyTier::Faang => "FAANG",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    #[test]
    fn test_job_match_fresh_player() {
        let player = Player::new("Test");
        let job = Job {
            id: 1,
            title: "Test Job".to_string(),
            company: "Test Co".to_string(),
            salary_min: 100000,
            salary_max: 150000,
            requirements: vec![
                SkillRequirement {
                    skill_name: "Python".to_string(),
                    min_proficiency: Proficiency::Intermediate,
                    mandatory: true,
                    weight: 1.0,
                },
            ],
            min_experience_days: 0,
            description: "A test job".to_string(),
            difficulty: 1,
        };
        
        let score = job.calculate_match(&player.skills);
        assert!(score < 0.5);
    }

    #[test]
    fn test_job_display_salary() {
        let job = Job {
            id: 1,
            title: "Test".to_string(),
            company: "Test".to_string(),
            salary_min: 100000,
            salary_max: 150000,
            requirements: vec![],
            min_experience_days: 0,
            description: "".to_string(),
            difficulty: 1,
        };
        
        assert_eq!(job.display_salary(), "$100000 - $150000/year");
    }

    #[test]
    fn test_company_tier_salary_multiplier() {
        assert!((CompanyTier::Startup.salary_multiplier() - 0.8).abs() < 0.01);
        assert!((CompanyTier::MidSize.salary_multiplier() - 1.0).abs() < 0.01);
        assert!((CompanyTier::BigTech.salary_multiplier() - 1.5).abs() < 0.01);
        assert!((CompanyTier::Faang.salary_multiplier() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_company_tier_ordering() {
        assert!(CompanyTier::Faang.difficulty_modifier() > CompanyTier::BigTech.difficulty_modifier());
        assert!(CompanyTier::BigTech.difficulty_modifier() > CompanyTier::MidSize.difficulty_modifier());
        assert!(CompanyTier::MidSize.difficulty_modifier() > CompanyTier::Startup.difficulty_modifier());
    }

    #[test]
    fn test_company_tier_as_str() {
        assert_eq!(CompanyTier::Startup.as_str(), "Startup");
        assert_eq!(CompanyTier::MidSize.as_str(), "Mid-Size");
        assert_eq!(CompanyTier::BigTech.as_str(), "Big Tech");
        assert_eq!(CompanyTier::Faang.as_str(), "FAANG");
    }
}
