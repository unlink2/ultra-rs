use super::memory::umemset;
use super::font::*;
use super::interrupt::{enable_int, disable_int};
use super::render::RenderContext;

// TODO remove hard coded ptrs
pub struct RdpFontRendererContext {
    buffer: *mut u32,
    pub offset: usize,
    start: usize,
    size: usize,
    registers: *mut u32,
}

impl RdpFontRendererContext {
    pub fn new(buffer: *mut u32, size: usize) -> Self {
        let mut rdp = Self {
            buffer,
            size,
            offset: 0,
            start: 0,
            registers: 0xA4100000 as *mut u32,
        };
        unsafe { rdp.clear_buffer(rdp.size); }
        rdp
    }

    #[inline(always)]
    unsafe fn wait_pipe(&mut self, other: u32) {
        while core::ptr::read_volatile(self.registers.add(3)) & (0x600 | other) != 0 {}
    }

    #[inline(always)]
    unsafe fn clear_buffer(&mut self, size: usize) {
        umemset(self.buffer.add(self.offset) as *mut u8, 0, size*4);
    }

    unsafe fn send_dl(&mut self, start: *mut u8, end: *mut u8) {
        if start == end { return; }

        // wait for rdp
        self.wait_pipe(0);

        // no interrupts to prevent conflicts
        disable_int();

        // set flush flag
        core::ptr::write_volatile(self.registers.add(3), 0x15);
        // wait again to flush
        self.wait_pipe(0);

        // start dma
        // & 0x00FFFFFF -> converts to physical address for dma
        core::ptr::write_volatile(self.registers.add(0), start as u32 & 0x00FFFFFF);
        core::ptr::write_volatile(self.registers.add(1), end as u32 & 0x00FFFFFF);
        self.wait_pipe(0);

        enable_int();
    }

    #[inline(always)]
    unsafe fn write(&mut self, offset: usize, value: u32) {
        *self.buffer.add(self.offset+offset) = value;
    }

    pub unsafe fn sync_full(&mut self) -> usize {
        let size: usize = 8;

        self.clear_buffer(size);

        self.write(0, 0x29000000);
        self.write(1, 0x00000000);

        return size;
    }

    pub unsafe fn sync_pipe(&mut self) -> usize {
        let size: usize = 8;

        self.clear_buffer(size);

        self.write(0, 0x27000000);
        self.write(1, 0x00000000);

        return size;
    }

    pub unsafe fn sync_load(&mut self) -> usize {
        let size: usize = 8;

        self.clear_buffer(size);

        self.write(0, 0x31000000);
        self.write(1, 0x00000000);

        return size;
    }

    pub unsafe fn sync_tile(&mut self) -> usize {
        let size: usize = 8;

        self.clear_buffer(size);

        self.write(0, 0x28000000);
        self.write(1, 0x00000000);

        return size;
    }

    pub unsafe fn draw_primitives(&mut self) -> usize {
        let size: usize = 8;

        self.clear_buffer(size);

        self.write(0, 0xEFB000FF);
        self.write(1, 0x00004004);

        return size;
    }

    pub unsafe fn draw_rect(&mut self, color: u32, tx: i32, ty: i32, bx: i32, by: i32) -> usize {
        let size: usize = 8;

        self.clear_buffer(size);

        // set color
        self.write(0, 0xF7000000);
        self.write(1, color);

        // set location
        self.write(2, 0xF6000000 | ( (bx as u32) << 14 ) | ( (by as u32) << 2 ));
        self.write(3, ( (tx as u32) << 14 ) | ( (ty as u32) << 2 ));

        return size;
    }

    pub unsafe fn texture_mode(&mut self) -> usize {
        let size: usize = 8;

        self.clear_buffer(size);

        self.write(0, 0x2F002830);
        self.write(1, 0x00404040);
        self.write(2, 0x3C0000C1);
        self.write(3, 0x0F2001FF);

        return size;
    }

    pub unsafe fn load_tile(&mut self, font: &dyn GenericFont, offset: usize) -> usize {
        let size: usize = 8;

        self.clear_buffer(size);

        // sync
        self.write(0, 0x27000000);
        self.write(1, 0x00000000);

        // set texture image
        self.write(2, 0x3D100007);
        self.write(3, font.data().as_ptr().add(offset) as u32);

        self.write(4, 0x31000000);
        self.write(5, 0x00000000);

        return size;
    }

    pub unsafe fn draw_tile(&mut self, xh: i32, yh: i32, w: i32, h: i32) -> usize {
        let size: usize = 8;

        self.clear_buffer(size);

        // set tile
        self.write(0, 0x35100400); //  0<<2,0<<2, 0, 7<<2,7<<2, sl 0.0 tl 0.0, sh 7.0 th 7.0
        self.write(1, 0x00000000);

        // set texture rect
        self.write(2, 0x34000000);
        self.write(3, 0x0001C01C);

        // draw rect
        self.write(4, 0x24000000  | (xh as u32 + w as u32) << 14 | (yh as u32 + h as u32) << 2); // xh+w == xl; yh+h == yl
        self.write(5, 0x00000000 | (xh as u32) << 14 | (yh as u32) << 2);
        self.write(6, 0x00000000); // S 0.0 T 0.0
        self.write(7, 0x04000400); // scale factor 1:1

        return size;
    }

}

impl RenderContext for RdpFontRendererContext {
    fn update(&mut self) {
        // needs to be divisible by 8!

        // clear difference
        let rem = 8 - self.offset % 8;
        if rem > 0 {
            unsafe { self.clear_buffer(rem); }
            self.offset += rem;
        }

        // send previous dl
        unsafe {
            self.send_dl((self.buffer.add(self.start)) as *mut u8,
                (self.buffer.add(self.offset)) as *mut u8);
        }
        // check bounds of dl
        if self.start > self.size {
            self.offset = 0;
            self.start = 0;
        }

        self.start = self.offset;
    }

    fn puts(&mut self, s: &str, x: i32, y: i32, font: &dyn GenericFont) {
        let mut current_x = x;
        unsafe {self.offset += self.texture_mode();}
        for c in s.chars() {
            self.draw_char(c, current_x, y, font);
            current_x += CHAR_W as i32;
        }
    }

    fn cputs(&mut self, s: &[char], x: i32, y: i32, font: &dyn GenericFont) {
        let mut current_x = x;
        unsafe {self.offset += self.texture_mode();}
        for c in s {
            if *c == '\0' { break; }
            self.draw_char(*c, current_x, y, font);
            current_x += CHAR_W as i32;
        }
    }

    fn draw_char(&mut self, c: char, x: i32, y: i32, font: &dyn GenericFont) {
        if c == '\0' { return; }

        let offset = c as usize
            * CHAR_W
            * CHAR_H;
        unsafe {
            self.offset += self.load_tile(font, offset);
            self.offset += self.draw_tile(x, y, CHAR_W as i32, CHAR_H as i32);
        }
    }
}
