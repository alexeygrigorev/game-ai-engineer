pub mod input;
pub mod canvas;
pub mod harness;

pub use input::{InputSnapshot, InputSource};
pub use canvas::{UiCanvas, MockCanvas, DrawOp, Color};
pub use harness::{TestHarness, ScriptedInput};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_input_snapshot_key_down() {
        let input = InputSnapshot::new().with_key_down("w");
        assert!(input.is_key_down("w"));
        assert!(!input.is_key_down("s"));
    }
    
    #[test]
    fn test_input_snapshot_key_pressed() {
        let input = InputSnapshot::new().with_key_pressed("e");
        assert!(input.is_key_pressed("e"));
        assert!(!input.is_key_pressed("w"));
    }
    
    #[test]
    fn test_input_snapshot_mouse() {
        let input = InputSnapshot::new()
            .with_mouse_pos(100.0, 200.0)
            .with_mouse_down();
        
        assert_eq!(input.mouse_x, 100.0);
        assert_eq!(input.mouse_y, 200.0);
        assert!(input.mouse_left_down);
    }
    
    #[test]
    fn test_mock_canvas_records_ops() {
        let mut canvas = MockCanvas::new();
        
        canvas.rect(10.0, 20.0, 100.0, 50.0, Color::RED);
        canvas.text("Hello", 50.0, 50.0, 20.0, Color::WHITE);
        
        assert_eq!(canvas.count_ops(), 2);
        assert_eq!(canvas.find_rects().len(), 1);
        assert_eq!(canvas.find_texts().len(), 1);
    }
    
    #[test]
    fn test_mock_canvas_find_text() {
        let mut canvas = MockCanvas::new();
        
        canvas.text("Day 1", 10.0, 10.0, 16.0, Color::WHITE);
        canvas.text("Energy: 100", 10.0, 30.0, 16.0, Color::WHITE);
        canvas.text("Money: $1000", 10.0, 50.0, 16.0, Color::WHITE);
        
        let energy_texts = canvas.find_text_containing("Energy");
        assert_eq!(energy_texts.len(), 1);
    }
    
    #[test]
    fn test_mock_canvas_clear() {
        let mut canvas = MockCanvas::new();
        
        canvas.rect(0.0, 0.0, 100.0, 100.0, Color::RED);
        assert_eq!(canvas.count_ops(), 1);
        
        canvas.clear();
        assert_eq!(canvas.count_ops(), 0);
    }
    
    #[test]
    fn test_input_snapshot_clear_pressed() {
        let mut input = InputSnapshot::new()
            .with_key_pressed("e")
            .with_mouse_pressed();
        
        assert!(input.is_key_pressed("e"));
        assert!(input.mouse_left_pressed);
        
        input.clear_pressed();
        
        assert!(!input.is_key_pressed("e"));
        assert!(!input.mouse_left_pressed);
    }
}
