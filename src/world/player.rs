use macroquad::prelude::*;
use crate::world::GameMap;
use crate::world::TILE_SIZE;

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct WorldPlayer {
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub walking: bool,
    pub anim_timer: f32,
}

impl WorldPlayer {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            direction: Direction::Down,
            walking: false,
            anim_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, map: &GameMap) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dy -= 1.0;
            self.direction = Direction::Up;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dy += 1.0;
            self.direction = Direction::Down;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dx -= 1.0;
            self.direction = Direction::Left;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dx += 1.0;
            self.direction = Direction::Right;
        }

        self.walking = dx != 0.0 || dy != 0.0;

        if self.walking {
            let len: f32 = (dx * dx + dy * dy) as f32;
            let len = len.sqrt();
            if len > 0.0 {
                dx /= len;
                dy /= len;
            }
            
            let new_x = self.x + dx * PLAYER_SPEED * dt;
            let new_y = self.y + dy * PLAYER_SPEED * dt;
            
            if !map.collides(new_x, self.y, PLAYER_SIZE, PLAYER_SIZE) {
                self.x = new_x;
            }
            if !map.collides(self.x, new_y, PLAYER_SIZE, PLAYER_SIZE) {
                self.y = new_y;
            }
            
            self.x = self.x.max(PLAYER_SIZE).min((crate::world::MAP_WIDTH as f32 - 1.0) * TILE_SIZE);
            self.y = self.y.max(PLAYER_SIZE).min((crate::world::MAP_HEIGHT as f32 - 1.0) * TILE_SIZE);
            
            self.anim_timer += dt;
        }
    }

    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn tile_position(&self) -> (i32, i32) {
        ((self.x / TILE_SIZE) as i32, (self.y / TILE_SIZE) as i32)
    }
}
