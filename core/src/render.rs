/// The following usage only applies when writing a custom rendering pipeline
/// if the game's font renderer is used you should be able to ignore the update function completely
/// for a sample see the WIP rdp font rendering context
pub trait RenderContext {
    /// this function should flush the
    /// last draw operations to the screen and
    /// clear the buffer
    fn draw(&mut self) {
    }

    /// the put and draw functions should write to
    /// an internal buffer and that gets drawn to
    /// the screen during update
    fn puts(&mut self, s: &str, x: i32, y: i32);
    fn cputs(&mut self, s: &[char], x: i32, y: i32);
}
