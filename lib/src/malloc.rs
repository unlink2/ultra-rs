use core::ffi::c_void;

#[repr(C)]
struct ListHead {
    next: *mut ListHead,
    prev: *mut ListHead,
}

impl ListHead {}

#[repr(C)]
struct AllocNode {
    head: ListHead,
    block: *mut u8,
    size: usize,
}

impl AllocNode {}
