use super::menu::*;
use super::render::RenderContext;
use core::ffi::c_void;

#[derive(Copy, Clone)]
pub struct Monitor<T>
where
    T: Copy + Clone,
{
    x: i32,
    y: i32,
    pub active: bool,

    addr: *mut c_void,
    amount: usize,
    toggle_timer_max: u16,
    toggle_timer: u16,
    close_action: Entry<T>,
    back_action: Entry<T>,
    open_action: Entry<T>,
}

impl<T> Monitor<T>
where
    T: Copy + Clone,
{
    pub fn new(
        x: i32,
        y: i32,
        open_action: Entry<T>,
        close_action: Entry<T>,
        back_action: Entry<T>,
    ) -> Self {
        Self {
            x,
            y,
            active: true,
            addr: 0x80000000 as *mut c_void,
            amount: 128,
            toggle_timer_max: 10,
            toggle_timer: 0,
            open_action,
            close_action,
            back_action,
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
        if !self.active {
            return;
        }
        ctxt.puts("Monitor", self.x, self.y);
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
}
