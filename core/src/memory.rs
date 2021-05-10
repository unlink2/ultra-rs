#[no_mangle]
pub unsafe extern "C" fn umemset(ptr: *mut u8, value: u8, size: isize) -> *mut u8 {
    for i in 0..size {
        *ptr.offset(i) = value;
    }
    return ptr;
}


