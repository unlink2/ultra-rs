use super::render::RenderContext;
use core::ffi::c_void;

pub struct Monitor {
    x: i32,
    y: i32,
    pub active: bool,

    addr: *mut c_void,
    amount: usize,
}

impl Monitor {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            active: false,
            addr: 0x80000000 as *mut c_void,
            amount: 128,
        }
    }
}
