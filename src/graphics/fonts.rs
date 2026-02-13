use macroquad::prelude::*;
use std::sync::OnceLock;

static FONT: OnceLock<Option<Font>> = OnceLock::new();
static mut USE_CUSTOM_FONT: bool = true;

pub fn init_fonts() {
    let font_data = include_bytes!("../../assets/PixelifySans-Regular.ttf");
    let font = load_ttf_font_from_bytes(font_data).ok();
    FONT.set(font).ok();
}

pub fn use_custom_font(enabled: bool) {
    unsafe {
        USE_CUSTOM_FONT = enabled;
    }
}

pub fn is_custom_font_enabled() -> bool {
    unsafe { USE_CUSTOM_FONT }
}

fn get_font() -> Option<&'static Font> {
    let custom = unsafe { USE_CUSTOM_FONT };
    if custom {
        FONT.get().and_then(|f| f.as_ref())
    } else {
        None
    }
}

pub fn draw_text_crisp(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let x = x.round();
    let y = y.round();
    let scale = 2.0;
    let size = (font_size * scale) as u16;

    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: get_font(),
            font_size: size,
            font_scale: 1.0 / scale,
            color,
            ..Default::default()
        },
    );
}

pub fn draw_text_crisp_centered(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let scale = 2.0;
    let size = (font_size * scale) as u16;

    let dims = measure_text(text, get_font(), size, 1.0 / scale);
    let x = (x - dims.width / 2.0).round();
    let y = y.round();

    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: get_font(),
            font_size: size,
            font_scale: 1.0 / scale,
            color,
            ..Default::default()
        },
    );
}
