#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(naked_functions)]
#![feature(asm_experimental_arch)]
use core::{arch::asm, panic::PanicInfo};

/**
 * # Safety
 * Barebones sample
 */
#[no_mangle]
#[naked]
#[link_section = ".boot"]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        r#"
        nop
        jal main
        nop
        b _start
        nop
    "#,
        options(noreturn)
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn main() {}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
