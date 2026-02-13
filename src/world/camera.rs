use macroquad::prelude::*;

pub struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn follow(&mut self, target_x: f32, target_y: f32) {
        let sw = screen_width();
        let sh = screen_height();
        self.x = target_x - sw / 2.0;
        self.y = target_y - sh / 2.0;
    }

    pub fn world_to_screen(&self, wx: f32, wy: f32) -> (f32, f32) {
        (wx - self.x, wy - self.y)
    }
}
