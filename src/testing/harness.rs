use crate::testing::input::{InputSnapshot, InputSource};
use crate::testing::canvas::{UiCanvas, MockCanvas};
use crate::player::Player;
use crate::skills::Proficiency;

pub struct TestHarness {
    pub player: Player,
    pub canvas: MockCanvas,
    pub frames: Vec<InputSnapshot>,
    pub current_frame: usize,
    pub elapsed_time: f32,
}

impl TestHarness {
    pub fn new() -> Self {
        Self {
            player: Player::new("TestPlayer"),
            canvas: MockCanvas::new(),
            frames: Vec::new(),
            current_frame: 0,
            elapsed_time: 0.0,
        }
    }
    
    pub fn with_player(mut self, player: Player) -> Self {
        self.player = player;
        self
    }
    
    pub fn add_frame(mut self, input: InputSnapshot) -> Self {
        self.frames.push(input);
        self
    }
    
    pub fn add_idle_frames(mut self, count: usize) -> Self {
        for _ in 0..count {
            self.frames.push(InputSnapshot::new());
        }
        self
    }
    
    pub fn add_movement_frames(mut self, direction: &str, count: usize) -> Self {
        for _ in 0..count {
            let input = InputSnapshot::new().with_key_down(direction);
            self.frames.push(input);
        }
        self
    }
    
    pub fn run_frame(&mut self, dt: f32) -> bool {
        if self.current_frame >= self.frames.len() {
            return false;
        }
        
        let input = self.frames[self.current_frame].clone();
        self.apply_input(&input, dt);
        self.current_frame += 1;
        self.elapsed_time += dt;
        true
    }
    
    pub fn run_all_frames(&mut self, dt: f32) {
        while self.run_frame(dt) {}
    }
    
    fn apply_input(&mut self, input: &InputSnapshot, _dt: f32) {
        if input.is_key_pressed("e") {
            // Handle interaction
        }
        
        if input.is_key_pressed("i") {
            // Open skills menu
        }
    }
    
    pub fn study_skill(&mut self, skill_name: &str, hours: u32) -> Result<String, String> {
        self.player.study(skill_name, hours)
    }
    
    pub fn rest(&mut self) {
        self.player.rest();
    }
    
    pub fn advance_day(&mut self) {
        self.player.advance_day();
    }
    
    pub fn get_canvas(&self) -> &MockCanvas {
        &self.canvas
    }
    
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.elapsed_time = 0.0;
        self.canvas.clear();
    }
}

pub struct ScriptedInput {
    frames: Vec<InputSnapshot>,
    current: usize,
}

impl ScriptedInput {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            current: 0,
        }
    }
    
    pub fn press_key(mut self, key: &str) -> Self {
        let frame = InputSnapshot::new().with_key_pressed(key);
        self.frames.push(frame);
        self
    }
    
    pub fn hold_key(mut self, key: &str, frames: usize) -> Self {
        for _ in 0..frames {
            let frame = InputSnapshot::new().with_key_down(key);
            self.frames.push(frame);
        }
        self
    }
    
    pub fn wait(mut self, frames: usize) -> Self {
        for _ in 0..frames {
            self.frames.push(InputSnapshot::new());
        }
        self
    }
    
    pub fn build(self) -> Vec<InputSnapshot> {
        self.frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_harness_creation() {
        let harness = TestHarness::new();
        assert_eq!(harness.player.name, "TestPlayer");
        assert_eq!(harness.player.energy, 100);
        assert_eq!(harness.player.money, 1000);
    }
    
    #[test]
    fn test_study_skill_in_harness() {
        let mut harness = TestHarness::new();
        
        let result = harness.study_skill("Python", 2);
        assert!(result.is_ok());
        assert_eq!(harness.player.energy, 80);
    }
    
    #[test]
    fn test_skill_leveling_in_harness() {
        let mut harness = TestHarness::new();
        
        for _ in 0..5 {
            let _ = harness.study_skill("Python", 4);
            harness.player.energy = 100;
        }
        
        let proficiency = harness.player.get_skill_proficiency("Python");
        assert!(proficiency >= Proficiency::Basic);
    }
    
    #[test]
    fn test_scripted_input() {
        let script = ScriptedInput::new()
            .press_key("e")
            .wait(5)
            .hold_key("w", 10)
            .build();
        
        assert_eq!(script.len(), 16);
        assert!(script[0].is_key_pressed("e"));
        assert!(script[6].is_key_down("w"));
    }
    
    #[test]
    fn test_rest_in_harness() {
        let mut harness = TestHarness::new();
        harness.player.energy = 50;
        
        harness.rest();
        
        assert_eq!(harness.player.energy, 100);
    }
    
    #[test]
    fn test_advance_day_in_harness() {
        let mut harness = TestHarness::new();
        let initial_day = harness.player.day;
        
        harness.advance_day();
        
        assert_eq!(harness.player.day, initial_day + 1);
    }
    
    #[test]
    fn test_employment_in_harness() {
        let mut harness = TestHarness::new();
        
        harness.player.employed = true;
        harness.player.current_salary = 100000;
        
        let initial_money = harness.player.money;
        harness.advance_day();
        
        assert!(harness.player.money > initial_money);
    }
    
    #[test]
    fn test_run_all_frames() {
        let mut harness = TestHarness::new()
            .add_idle_frames(10);
        
        harness.run_all_frames(1.0 / 60.0);
        
        assert_eq!(harness.current_frame, 10);
    }
}
