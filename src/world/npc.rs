use macroquad::prelude::*;
use crate::graphics::draw_npc;

#[derive(Debug, Clone)]
pub enum NpcType {
    Recruiter,
    Engineer,
    Student,
    Professor,
    Barista,
}

impl NpcType {
    pub fn name(&self) -> &str {
        match self {
            NpcType::Recruiter => "Recruiter",
            NpcType::Engineer => "Senior Engineer",
            NpcType::Student => "Student",
            NpcType::Professor => "Professor",
            NpcType::Barista => "Barista",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Npc {
    pub x: f32,
    pub y: f32,
    pub npc_type: NpcType,
    pub name: String,
    pub dialog: Vec<String>,
    pub current_dialog: usize,
}

impl Npc {
    pub fn new(x: f32, y: f32, npc_type: NpcType) -> Self {
        let (name, dialog) = match &npc_type {
            NpcType::Recruiter => (
                "Alex".to_string(),
                vec![
                    "Hey! I'm a recruiter from a tech company.".to_string(),
                    "We're always looking for talented AI engineers.".to_string(),
                    "Make sure your skills are up to date before applying!".to_string(),
                ]
            ),
            NpcType::Engineer => (
                "Jordan".to_string(),
                vec![
                    "I've been in the AI field for 10 years.".to_string(),
                    "My advice? Focus on fundamentals first.".to_string(),
                    "Transformers are hot right now, but understanding the basics is key.".to_string(),
                ]
            ),
            NpcType::Student => (
                "Sam".to_string(),
                vec![
                    "I'm also trying to break into AI!".to_string(),
                    "The library has great resources for studying.".to_string(),
                    "Good luck with your job search!".to_string(),
                ]
            ),
            NpcType::Professor => (
                "Dr. Chen".to_string(),
                vec![
                    "Welcome! I teach the advanced ML course.".to_string(),
                    "If you want to master LLMs, you need strong foundations.".to_string(),
                    "Come back when you've studied the basics.".to_string(),
                ]
            ),
            NpcType::Barista => (
                "Morgan".to_string(),
                vec![
                    "Welcome to the Coffee Shop!".to_string(),
                    "Coffee gives you energy, and it's a great place to network.".to_string(),
                    "I've seen many developers land jobs through connections here!".to_string(),
                ]
            ),
        };

        Self {
            x,
            y,
            npc_type,
            name,
            dialog,
            current_dialog: 0,
        }
    }

    pub fn npc_type_id(&self) -> u8 {
        match self.npc_type {
            NpcType::Recruiter => 0,
            NpcType::Engineer => 1,
            NpcType::Student => 2,
            NpcType::Professor => 3,
            NpcType::Barista => 4,
        }
    }

    pub fn draw(&self) {
        draw_npc(self.x, self.y, self.npc_type_id());
    }

    pub fn distance_to(&self, px: f32, py: f32) -> f32 {
        let dx = self.x - px;
        let dy = self.y - py;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn get_dialog(&self) -> (&str, &str) {
        let text = self.dialog.get(self.current_dialog).unwrap_or(&self.dialog[0]);
        (self.name.as_str(), text.as_str())
    }

    pub fn advance_dialog(&mut self) -> bool {
        self.current_dialog += 1;
        self.current_dialog < self.dialog.len()
    }

    pub fn reset_dialog(&mut self) {
        self.current_dialog = 0;
    }
}

pub fn get_npcs() -> Vec<Npc> {
    vec![
        Npc::new(10.0 * 32.0, 9.0 * 32.0, NpcType::Recruiter),
        Npc::new(7.0 * 32.0, 16.0 * 32.0, NpcType::Engineer),
        Npc::new(21.0 * 32.0, 16.0 * 32.0, NpcType::Student),
        Npc::new(19.0 * 32.0, 12.0 * 32.0, NpcType::Professor),
        Npc::new(22.0 * 32.0, 14.0 * 32.0, NpcType::Barista),
    ]
}
