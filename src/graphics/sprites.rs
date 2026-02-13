use super::draw_text_crisp;
use crate::world::Direction;
use crate::world::TILE_SIZE;
use macroquad::prelude::*;

pub fn draw_player(x: f32, y: f32, direction: Direction, walking: bool, anim_timer: f32) {
    let bounce = if walking {
        (anim_timer * 10.0).sin() * 3.0
    } else {
        0.0
    };

    let px = x;
    let py = y + bounce;

    draw_rectangle(px - 10.0, py - 20.0, 20.0, 12.0, BROWN);
    draw_circle(px, py - 5.0, 10.0, BEIGE);
    draw_rectangle(px - 12.0, py + 5.0, 24.0, 18.0, BLUE);
    draw_rectangle(px - 10.0, py + 23.0, 8.0, 12.0, DARKGRAY);
    draw_rectangle(px + 2.0, py + 23.0, 8.0, 12.0, DARKGRAY);

    let eye_offset = match direction {
        Direction::Left => -4.0,
        Direction::Right => 4.0,
        _ => 0.0,
    };
    draw_circle(px + eye_offset - 4.0, py - 5.0, 2.0, BLACK);
    draw_circle(px + eye_offset + 4.0, py - 5.0, 2.0, BLACK);
}

pub fn draw_npc(x: f32, y: f32, npc_type: u8) {
    let colors = [RED, GREEN, BLUE, PURPLE, ORANGE];
    let body_color = colors[(npc_type % 5) as usize];

    draw_rectangle(x - 10.0, y - 20.0, 20.0, 12.0, BROWN);
    draw_circle(x, y - 5.0, 10.0, BEIGE);
    draw_rectangle(x - 12.0, y + 5.0, 24.0, 18.0, body_color);
    draw_rectangle(x - 10.0, y + 23.0, 8.0, 12.0, DARKGRAY);
    draw_rectangle(x + 2.0, y + 23.0, 8.0, 12.0, DARKGRAY);
}

pub fn draw_grass_tile(x: f32, y: f32) {
    draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, DARKGREEN);
}

pub fn draw_path_tile(x: f32, y: f32) {
    draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, GRAY);
}

pub fn draw_building(x: f32, y: f32, width: u32, height: u32, name: &str, color: Color) {
    let w = width as f32 * TILE_SIZE;
    let h = height as f32 * TILE_SIZE;

    draw_rectangle(x, y, w, h, color);
    draw_rectangle(x, y, w, 10.0, DARKBROWN);

    for col in 0..width {
        let wx = x + 8.0 + col as f32 * TILE_SIZE;
        let wy = y + 15.0;
        if wx + 16.0 < x + w - 8.0 && wy + 16.0 < y + h - 15.0 {
            draw_rectangle(wx, wy, 16.0, 16.0, LIGHTGRAY);
            draw_line(wx + 8.0, wy, wx + 8.0, wy + 16.0, 2.0, GRAY);
            draw_line(wx, wy + 8.0, wx + 16.0, wy + 8.0, 2.0, GRAY);
        }
    }

    let door_x = x + w / 2.0 - 10.0;
    let door_y = y + h - 28.0;
    draw_rectangle(door_x, door_y, 20.0, 28.0, BROWN);

    draw_text_crisp(name, x + 5.0, y + h + 15.0, 16.0, WHITE);
}

pub fn draw_library(x: f32, y: f32) {
    draw_building(x, y, 4, 3, "Library", Color::from_rgba(139, 90, 43, 255));
}

pub fn draw_company(x: f32, y: f32, name: &str, tier: u8) {
    let color = match tier {
        0 => GREEN,
        1 => BLUE,
        2 => PURPLE,
        3 => RED,
        _ => GRAY,
    };
    let width = 3 + tier as u32;
    let height = 2 + tier as u32;
    draw_building(x, y, width, height, name, color);
}

pub fn draw_apartment(x: f32, y: f32) {
    draw_building(x, y, 3, 2, "Apartment", GRAY);
}

pub fn draw_coffee_shop(x: f32, y: f32) {
    draw_building(x, y, 3, 2, "Coffee", BROWN);
}

pub fn draw_park(x: f32, y: f32, width: u32, height: u32) {
    let w = width as f32 * TILE_SIZE;
    let h = height as f32 * TILE_SIZE;
    draw_rectangle(x, y, w, h, GREEN);
}
