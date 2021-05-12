#[no_mangle]
pub unsafe extern "C" fn umemset(ptr: *mut u8, value: u8, size: usize) -> *mut u8 {
    let mut i = 0;
    while i < size {
        *ptr.add(i) = value;
        i += 1;
    }
    return ptr;
}


