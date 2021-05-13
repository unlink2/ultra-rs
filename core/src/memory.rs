#[no_mangle]
pub unsafe extern "C" fn umemset(ptr: *mut u8, value: u8, size: usize) -> *mut u8 {
    let mut i = 0;
    while i < size {
        *ptr.add(i) = value;
        i += 1;
    }
    return ptr;
}

/**
 * Simple shared reference implementation
 * no bells and whistles, just a way to pass
 * values with shared ownership
 * since there is no allocator yet we use an
 * unsafe pointer for now!
 * Once we can use something similar the Box use it!
 */
#[derive(Copy, Clone)]
pub struct SharedPtrCell<T> {
    ptr: *mut T
}

impl<'a, T> SharedPtrCell<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self {
            ptr
        }
    }

    /**
     * Of course this is so unsafe it's not even funny
     * but we should not usually panic!
     */
    pub fn as_ref(&self) -> &'a T {
        unsafe {
            return self.ptr.as_ref().unwrap();
        }
    }

    pub fn as_mut(&mut self) -> &'a T {
        unsafe {
            return self.ptr.as_mut().unwrap();
        }
    }
}



