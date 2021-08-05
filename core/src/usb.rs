// Ed64 usb Code
use core::ffi::c_void;
use crate::embedgdb::Parser;

const REG_BASE: usize = 0x1F800000;
const ED_BASE_REG: usize =0xBF800000;
const REG_KEY: usize = 0x8004;
const REG_SYS_CFG: usize = 0x8000;

const PI_BASE_REG: usize = 0x04600000;
const PI_BSD_DOM1_LAT_REG: usize = PI_BASE_REG+0x14;
const PI_BSD_DOM1_PWD_REG: usize = PI_BASE_REG+0x18;
const PI_BSD_DOM1_PGS_REG: usize = PI_BASE_REG+0x1C;    /*   page size */
const PI_BSD_DOM1_RLS_REG: usize = PI_BASE_REG+0x20;
const PI_BSD_DOM2_LAT_REG: usize = PI_BASE_REG+0x24;    /* Domain 2 latency */
const PI_BSD_DOM2_PWD_REG: usize = PI_BASE_REG+0x28;    /*   pulse width */
const PI_BSD_DOM2_PGS_REG: usize = PI_BASE_REG+0x2C;    /*   page size */
const PI_BSD_DOM2_RLS_REG: usize = PI_BASE_REG+0x30;    /*   release duration */
const PI_STATUS_REG: usize = PI_BASE_REG+0x10;

const PI_STATUS_DMA_BUSY: usize = 1 << 0;
const PI_STATUS_IO_BUSY: usize  = 1 << 1;
const PI_STATUS_ERROR: usize    = 1 << 2;
const KSEG1: usize = 0xA0000000;


const SAVE_OFF: usize      =  0;
const SAVE_EEP4K: usize    =  1;
const SAVE_EEP16K: usize   =  2;
const SAVE_SRM32K: usize   =  3;
const SAVE_SRM96K: usize   =  4;
const SAVE_FLASH: usize    =  5;
const SAVE_SRM128K: usize  =  6;
const SAVE_MPAK: usize     =  8;
const SAVE_DD64: usize     =  16;

struct pi_regs {
    ram_addr: *mut c_void,
    pi_addr: *mut c_void,
    read_len: usize,
    write_len: usize,
    status: usize
}

/// we need the registers at this specific address
const PIR: *mut pi_regs = PI_BASE_REG as *mut pi_regs;

#[inline]
unsafe fn ed_regs(reg: *mut u32) -> *mut u32 {
    (KSEG1 | REG_BASE | reg as usize) as *mut u32
}

#[inline]
unsafe fn phys_to_k1(addr: *mut u32) -> *mut u32 {
    (addr as usize | KSEG1) as *mut u32
}

#[inline]
unsafe fn io_write(addr: *mut u32, data: u32) {
   *phys_to_k1(addr) = data;
}

#[inline]
unsafe fn io_read(addr: *mut u32) -> u32 {
    *phys_to_k1(addr)
}

#[inline]
unsafe fn cache_op(op: usize, mut addr: *mut c_void, len: usize) {
    addr = (addr as usize & !3) as *mut c_void;

    for _ in (0..len).rev() {
        addr = addr.offset(4);
        asm!(r#"
            tcache {}, ({})
        "#, in(reg) op, in(reg) addr);
    }
}

#[inline]
unsafe fn data_cache_hit_writeback(addr: *mut c_void, len: usize) {
    cache_op(0x19, addr, len);
}

#[inline]
unsafe fn data_cache_hit_writeback_invalidate(addr: *mut c_void, len: usize) {
    cache_op(0x15, addr, len);
}

#[inline]
unsafe fn dma_busy() -> bool {
    (*PIR).status & (PI_STATUS_DMA_BUSY | PI_STATUS_IO_BUSY) != 0
}

unsafe fn pi_read(ram: *mut c_void, mut pi_addr: *mut c_void, len: usize) {
    pi_addr = (pi_addr as usize & 0x1FFFFFFF) as *mut c_void;
    data_cache_hit_writeback_invalidate(ram, len);

    while dma_busy() {}
    io_write(PI_STATUS_REG as *mut u32, 3);
    (*PIR).ram_addr = ram;
    (*PIR).pi_addr = pi_addr;
    (*PIR).write_len = len-1;
    while dma_busy() {}
}

unsafe fn pi_write(ram: *mut c_void, mut pi_addr: *mut c_void, len: usize) {
    pi_addr = (pi_addr as usize & 0x1FFFFFFF) as *mut c_void;
    data_cache_hit_writeback(ram, len);

    while dma_busy() {}
    io_write(PI_STATUS_REG as *mut u32, 3);
    (*PIR).ram_addr = ram;
    (*PIR).pi_addr = pi_addr;
    (*PIR).read_len = len-1;
    while dma_busy() {}
}

unsafe fn evd_reg_write(reg: *mut u32, val: u32) {
    let preg = ed_regs(reg);
    *preg = val;
}

unsafe fn evd_reg_read(reg: *mut u32) -> u32 {
    let preg = ed_regs(reg);
    *preg
}

unsafe fn evd_init() {
    io_write(PI_BSD_DOM1_LAT_REG as *mut u32, 0x04);
    io_write(PI_BSD_DOM1_PWD_REG as *mut u32, 0x0C);

    // unlock everdrive
    evd_reg_write(REG_KEY as *mut u32, 0xAA55);
    evd_reg_write(REG_SYS_CFG as *mut u32, 0);

    // TODO
    // evd_usb_init();
    // evd_set_save_type(SAVE_OFF);
}
