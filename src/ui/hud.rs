use macroquad::prelude::*;
use crate::game::GameState;

pub fn draw_hud(state: &GameState) {
    let font_size = 20.0;
    let mut x = 15.0;
    let y = 25.0;

    draw_text(&format!("Day {}", state.day), x, y, font_size, WHITE);
    x += 80.0;

    draw_text(&state.time_string(), x, y, font_size, LIGHTGRAY);
    x += 70.0;

    let energy_color = if state.player.energy < 30 {
        RED
    } else {
        GREEN
    };
    draw_text(&format!("Energy: {}/{}", state.player.energy, state.player.max_energy), x, y, font_size, energy_color);
    x += 140.0;

    draw_text(&format!("${}", state.player.money), x, y, font_size, GOLD);
    x += 90.0;

    if state.player.employed {
        draw_text(&format!("EMPLOYED ${}/yr", state.player.current_salary), x, y, font_size, LIME);
    }
}

pub fn draw_interaction_hint(text: &str) {
    let y = screen_height() - 60.0;
    let font_size = 18.0;
    draw_text(text, 10.0, y, font_size, YELLOW);
}

pub fn draw_controls_hint() {
    let text = "WASD: Move | E: Interact | I: Skills | J: Jobs | ESC: Menu";
    let y = screen_height() - 20.0;
    draw_text(text, 10.0, y, 14.0, GRAY);
}
