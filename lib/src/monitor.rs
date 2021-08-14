use super::color::Color;
use super::embedgdb::Parser;
use super::menu::*;
use super::render::RenderContext;
use core::ffi::c_void;

#[derive(Copy, Clone)]
pub struct Monitor<T>
where
    T: Copy + Clone,
{
    x: isize,
    y: isize,
    pub active: bool,

    addr: *mut c_void,
    rows: usize,
    bytes_per_row: usize,
    toggle_timer_max: u16,
    toggle_timer: u16,
    close_action: Entry<T>,
    back_action: Entry<T>,
    open_action: Entry<T>,
    cursor_x: usize,
    cursor_y: usize,
}

impl<T> Monitor<T>
where
    T: Copy + Clone,
{
    pub fn new(
        x: isize,
        y: isize,
        open_action: Entry<T>,
        close_action: Entry<T>,
        back_action: Entry<T>,
    ) -> Self {
        Self {
            x,
            y,
            active: true,
            addr: 0x80000000 as *mut c_void,
            bytes_per_row: 8,
            rows: 8,
            toggle_timer_max: 10,
            toggle_timer: 0,
            open_action,
            close_action,
            back_action,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn update(&mut self) {
        if self.toggle_timer > 0 {
            self.toggle_timer -= 1;
        }
        if !self.active {
            return;
        }
    }

    pub fn draw(&mut self, ctxt: &mut dyn RenderContext) {
        ctxt.puts("Monitor", self.x, self.y);

        for r in 0..self.rows {
            let offset_hex = Parser::to_hex_tuple(self.calc_offset(0, r) as u8);

            let addr_offset = [offset_hex.0 as char, offset_hex.1 as char];
            let y = self.y as isize + 8 * (r as isize + 1);

            ctxt.set_color(Color::new(0xFF, 0xFF, 0x00, 0xFF));
            ctxt.cputs(&addr_offset, self.x as isize, y);

            for c in 0..self.bytes_per_row {
                let address = unsafe { self.addr.add(self.calc_offset(c, r)) };
                let value = if address as usize >= 0x80000000 && (address as usize) < 0x807FFFFF {
                    unsafe { *(address as *const u8) }
                } else {
                    0
                };

                let x = self.x as isize + 24 * (c as isize + 1);

                if self.cursor_x == c && self.cursor_y == r {
                    ctxt.set_color(Color::new(0xFF, 0x00, 0x00, 0xFF));
                }

                let value_hex = Parser::to_hex_tuple(value);
                let value_str = [value_hex.0 as char, value_hex.1 as char];
                ctxt.cputs(&value_str, x, y);
            }
        }
    }

    pub fn left(&mut self) {
        if self.cursor_x == 0 {
            self.cursor_x = self.bytes_per_row - 1;
        } else {
            self.cursor_x -= 1;
        }
    }

    pub fn right(&mut self) {
        self.cursor_x += 1;
        if self.cursor_x >= self.bytes_per_row {
            self.cursor_x = 0;
        }
    }

    pub fn up(&mut self) {
        if self.cursor_y == 0 {
            self.cursor_y = self.rows - 1;
        } else {
            self.cursor_y -= 1;
        }
    }

    pub fn down(&mut self) {
        self.cursor_y += 1;
        if self.cursor_y >= self.rows {
            self.cursor_y = 0;
        }
    }

    pub fn inc_addr(&mut self) {
        self.addr = unsafe { self.addr.add(0x10) };
    }

    pub fn dec_addr(&mut self) {
        self.addr = unsafe { self.addr.sub(0x10) };
    }

    pub fn inc_value(&mut self) {
        let addr = unsafe {
            self.addr
                .add(self.calc_offset(self.cursor_x, self.cursor_y))
        };

        unsafe {
            *(addr as *mut u8) += 1;
        }
    }

    pub fn dec_value(&mut self) {
        let addr = unsafe {
            self.addr
                .add(self.calc_offset(self.cursor_x, self.cursor_y))
        };

        unsafe {
            *(addr as *mut u8) -= 1;
        }
    }

    pub fn open(&mut self, data: T) {
        self.active = true;
        self.open_action.activate(data);
    }

    pub fn close(&mut self, data: T) {
        self.active = false;
        self.close_action.activate(data);
    }

    pub fn back(&mut self, data: T) {
        self.toggle_timer = 0;
        self.back_action.activate(data);
    }

    pub fn toggle(&mut self, data: T) {
        if self.toggle_timer > 0 {
            return;
        }

        self.toggle_timer = self.toggle_timer_max;
        if self.active {
            self.close(data);
        } else {
            self.open(data);
        }
    }

    fn calc_offset(&self, x: usize, y: usize) -> usize {
        y * self.bytes_per_row + x
    }
}
