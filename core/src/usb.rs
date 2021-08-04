// Ed64 usb Code

#[inline]
unsafe fn cache_op(op: usize, mut addr: *mut usize, len: usize) {
    addr = (addr as usize & !3) as *mut usize;

    for _ in (0..len).rev() {
        addr = addr.offset(4);
        asm!(r#"
            tcache {}, ({})
        "#, in(reg) op, in(reg) addr);
    }
}

#[inline]
unsafe fn data_cache_hit_writeback(addr: *mut usize, len: usize) {
    cache_op(0x19, addr, len);
}

unsafe fn data_cache_hit_wrieback_invalidate(addr: *mut usize, len: usize) {
    cache_op(0x15, addr, len);
}

unsafe dma_busy() -> bool {
    false
}
