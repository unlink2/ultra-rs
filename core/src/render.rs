use super::font::GenericFont;

pub trait RenderContext {
    fn update(&mut self);
    fn puts(&mut self, s: &str, x: i32, y: i32, font: &dyn GenericFont);
    fn cputs(&mut self, s: &[char], x: i32, y: i32, font: &dyn GenericFont);
    fn draw_char(&mut self, c: char, x: i32, y: i32, font: &dyn GenericFont);
}
