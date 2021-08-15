use super::render::{Drawable, RenderContext};

#[derive(Copy, Clone)]
pub struct Keyboard {
    cursor_x: usize,
    cursor_y: usize,
    active: bool,
}

impl Keyboard {}

impl<T> Drawable<T> for Keyboard
where
    T: Copy + Clone,
{
    fn draw(&mut self, ctxt: &mut dyn RenderContext) {}

    fn update(&mut self, data: T) {}
}
