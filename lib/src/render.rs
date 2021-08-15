use crate::color::Color;

/// The following usage only applies when writing a custom rendering pipeline
/// if the game's font renderer is used you should be able to ignore the update function completely
/// for a sample see the WIP rdp font rendering context
pub trait RenderContext {
    /// this function should flush the
    /// last draw operations to the screen and
    /// clear the buffer
    fn draw(&mut self) {}

    /// the put and draw functions should write to
    /// an internal buffer and that gets drawn to
    /// the screen during update
    /// puts a rust string slice to the screen
    fn puts(&mut self, s: &str, x: isize, y: isize);

    /// puts an array of chars
    fn cputs(&mut self, s: &[char], x: isize, y: isize);

    /// puts an array of raw bytes (c-like, null terminated )
    fn putsu8(&mut self, s: &[u8], x: isize, y: isize);

    /// optional method, can be ignored
    /// if not supported
    /// return false whenever color is not supported
    fn set_color(&mut self, _color: Color) -> bool {
        false
    }

    fn char_width(&self) -> isize {
        10
    }

    fn char_height(&self) -> isize {
        10
    }
}

pub trait Drawable<T>
where
    T: Copy + Clone,
{
    fn draw(&mut self, ctxt: &mut dyn RenderContext);

    fn update(&mut self, data: T);
}

pub trait Widget<T>
where
    T: Copy + Clone,
{
    fn toggle(&mut self, data: T);
    fn active(&self) -> bool;
}
