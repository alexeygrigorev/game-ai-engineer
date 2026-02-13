use std::collections::HashMap;

use crate::skills::{get_all_skills, Proficiency, Skill, SkillCategory};

#[derive(Debug, Clone)]
pub struct PlayerSkill {
    pub skill: Skill,
    pub proficiency: Proficiency,
    pub experience_points: u32,
}

impl PlayerSkill {
    pub fn new(skill: Skill) -> Self {
        Self {
            skill,
            proficiency: Proficiency::None,
            experience_points: 0,
        }
    }

    pub fn points_to_next_level(&self) -> u32 {
        (self.skill.difficulty as u32) * 100
    }

    pub fn add_experience(&mut self, points: u32) -> bool {
        self.experience_points += points;
        let needed = self.points_to_next_level();
        if self.experience_points >= needed {
            if let Some(next) = self.proficiency.next() {
                self.proficiency = next;
                self.experience_points -= needed;
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub skills: HashMap<String, PlayerSkill>,
    pub money: u32,
    pub energy: u32,
    pub max_energy: u32,
    pub day: u32,
    pub employed: bool,
    pub current_salary: u32,
    pub reputation: u32,
}

impl Player {
    pub fn new(name: &str) -> Self {
        let mut skills = HashMap::new();
        for skill in get_all_skills() {
            skills.insert(skill.name.clone(), PlayerSkill::new(skill));
        }

        Self {
            name: name.to_string(),
            skills,
            money: 1000,
            energy: 100,
            max_energy: 100,
            day: 1,
            employed: false,
            current_salary: 0,
            reputation: 0,
        }
    }

    pub fn rest(&mut self) {
        self.energy = self.max_energy;
    }

    pub fn study(&mut self, skill_name: &str, hours: u32) -> Result<String, String> {
        let energy_cost = hours * 10;
        if self.energy < energy_cost {
            return Err("Not enough energy to study".to_string());
        }

        if let Some(player_skill) = self.skills.get_mut(skill_name) {
            self.energy -= energy_cost;
            let xp_gained = hours * 25;
            let leveled_up = player_skill.add_experience(xp_gained);
            
            if leveled_up {
                Ok(format!(
                    "Studied {} for {} hours. Level up! Now at {}",
                    skill_name, hours, player_skill.proficiency.as_str()
                ))
            } else {
                let needed = player_skill.points_to_next_level();
                let remaining = needed.saturating_sub(player_skill.experience_points);
                Ok(format!(
                    "Studied {} for {} hours. {} XP to next level",
                    skill_name, hours, remaining
                ))
            }
        } else {
            Err(format!("Unknown skill: {}", skill_name))
        }
    }

    pub fn get_skill_proficiency(&self, skill_name: &str) -> Proficiency {
        self.skills
            .get(skill_name)
            .map(|s| s.proficiency)
            .unwrap_or(Proficiency::None)
    }

    pub fn advance_day(&mut self) {
        self.day += 1;
        if self.employed {
            self.money += self.current_salary / 22;
        }
    }

    pub fn get_skills_by_category(&self) -> HashMap<SkillCategory, Vec<(&String, &PlayerSkill)>> {
        let mut by_category: HashMap<SkillCategory, Vec<(&String, &PlayerSkill)>> = HashMap::new();
        for (name, skill) in &self.skills {
            by_category
                .entry(skill.skill.category)
                .or_default()
                .push((name, skill));
        }
        by_category
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("TestPlayer");
        assert_eq!(player.name, "TestPlayer");
        assert_eq!(player.energy, 100);
        assert_eq!(player.money, 1000);
        assert!(!player.employed);
        assert!(player.skills.len() > 0);
    }

    #[test]
    fn test_study_reduces_energy() {
        let mut player = Player::new("Test");
        let initial_energy = player.energy;
        let result = player.study("Python", 2);
        assert!(result.is_ok());
        assert_eq!(player.energy, initial_energy - 20);
    }

    #[test]
    fn test_study_unknown_skill() {
        let mut player = Player::new("Test");
        let result = player.study("NonexistentSkill", 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_study_not_enough_energy() {
        let mut player = Player::new("Test");
        player.energy = 5;
        let result = player.study("Python", 2);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Not enough energy"));
    }

    #[test]
    fn test_skill_level_up() {
        let skill = get_all_skills().into_iter().find(|s| s.name == "Python").unwrap();
        let mut player_skill = PlayerSkill::new(skill);
        
        assert_eq!(player_skill.proficiency, Proficiency::None);
        
        let leveled = player_skill.add_experience(100);
        assert!(leveled);
        assert_eq!(player_skill.proficiency, Proficiency::Basic);
    }

    #[test]
    fn test_skill_partial_xp() {
        let skill = get_all_skills().into_iter().find(|s| s.name == "Python").unwrap();
        let mut player_skill = PlayerSkill::new(skill);
        
        let leveled = player_skill.add_experience(50);
        assert!(!leveled);
        assert_eq!(player_skill.proficiency, Proficiency::None);
        assert_eq!(player_skill.experience_points, 50);
    }

    #[test]
    fn test_rest() {
        let mut player = Player::new("Test");
        player.energy = 50;
        player.rest();
        assert_eq!(player.energy, player.max_energy);
    }

    #[test]
    fn test_advance_day() {
        let mut player = Player::new("Test");
        let initial_day = player.day;
        player.advance_day();
        assert_eq!(player.day, initial_day + 1);
    }

    #[test]
    fn test_employed_salary() {
        let mut player = Player::new("Test");
        player.employed = true;
        player.current_salary = 100000;
        let initial_money = player.money;
        player.advance_day();
        assert!(player.money > initial_money);
    }
}
