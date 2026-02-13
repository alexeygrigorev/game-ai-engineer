use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
pub struct InputSnapshot {
    pub keys_down: HashSet<String>,
    pub keys_pressed: HashSet<String>,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_left_down: bool,
    pub mouse_left_pressed: bool,
}

impl InputSnapshot {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_left_down: false,
            mouse_left_pressed: false,
        }
    }

    pub fn with_key_down(mut self, key: &str) -> Self {
        self.keys_down.insert(key.to_lowercase());
        self
    }

    pub fn with_key_pressed(mut self, key: &str) -> Self {
        self.keys_pressed.insert(key.to_lowercase());
        self
    }

    pub fn with_mouse_pos(mut self, x: f32, y: f32) -> Self {
        self.mouse_x = x;
        self.mouse_y = y;
        self
    }

    pub fn with_mouse_down(mut self) -> Self {
        self.mouse_left_down = true;
        self
    }

    pub fn with_mouse_pressed(mut self) -> Self {
        self.mouse_left_pressed = true;
        self
    }

    pub fn is_key_down(&self, key: &str) -> bool {
        self.keys_down.contains(&key.to_lowercase())
    }

    pub fn is_key_pressed(&self, key: &str) -> bool {
        self.keys_pressed.contains(&key.to_lowercase())
    }

    pub fn clear_pressed(&mut self) {
        self.keys_pressed.clear();
        self.mouse_left_pressed = false;
    }
}

pub trait InputSource {
    fn snapshot(&mut self) -> InputSnapshot;
}

pub struct ScriptedInputSource {
    frames: Vec<InputSnapshot>,
    current: usize,
}

impl ScriptedInputSource {
    pub fn new(frames: Vec<InputSnapshot>) -> Self {
        Self { frames, current: 0 }
    }
}

impl InputSource for ScriptedInputSource {
    fn snapshot(&mut self) -> InputSnapshot {
        if self.current < self.frames.len() {
            let snapshot = self.frames[self.current].clone();
            self.current += 1;
            snapshot
        } else {
            InputSnapshot::new()
        }
    }
}
