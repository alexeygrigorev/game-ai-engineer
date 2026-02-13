#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
}

#[derive(Clone, Debug, PartialEq)]
pub enum DrawOp {
    Rect { x: f32, y: f32, w: f32, h: f32, color: Color },
    Circle { x: f32, y: f32, r: f32, color: Color },
    Line { x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color },
    Text { text: String, x: f32, y: f32, size: f32, color: Color },
}

pub trait UiCanvas {
    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color);
    fn circle(&mut self, x: f32, y: f32, r: f32, color: Color);
    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color);
    fn text(&mut self, text: &str, x: f32, y: f32, size: f32, color: Color);
    fn clear(&mut self);
}

#[derive(Clone, Debug, Default)]
pub struct MockCanvas {
    pub ops: Vec<DrawOp>,
}

impl MockCanvas {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }
    
    pub fn find_rects(&self) -> Vec<&DrawOp> {
        self.ops.iter().filter(|op| matches!(op, DrawOp::Rect { .. })).collect()
    }
    
    pub fn find_texts(&self) -> Vec<&DrawOp> {
        self.ops.iter().filter(|op| matches!(op, DrawOp::Text { .. })).collect()
    }
    
    pub fn find_text_containing(&self, search: &str) -> Vec<&DrawOp> {
        self.ops.iter().filter(|op| {
            if let DrawOp::Text { text, .. } = op {
                text.contains(search)
            } else {
                false
            }
        }).collect()
    }
    
    pub fn count_ops(&self) -> usize {
        self.ops.len()
    }
}

impl UiCanvas for MockCanvas {
    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.ops.push(DrawOp::Rect { x, y, w, h, color });
    }
    
    fn circle(&mut self, x: f32, y: f32, r: f32, color: Color) {
        self.ops.push(DrawOp::Circle { x, y, r, color });
    }
    
    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
        self.ops.push(DrawOp::Line { x1, y1, x2, y2, thickness, color });
    }
    
    fn text(&mut self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        self.ops.push(DrawOp::Text { text: text.to_string(), x, y, size, color });
    }
    
    fn clear(&mut self) {
        self.ops.clear();
    }
}
