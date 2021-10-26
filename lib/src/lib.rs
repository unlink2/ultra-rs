#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(naked_functions)]

extern crate embedgdb;

pub mod clone;
pub mod color;
pub mod font;
pub mod frameadvance;
pub mod input;
pub mod interrupt;
pub mod keyboard;
pub mod malloc;
pub mod math;
pub mod memory;
pub mod menu;
pub mod monitor;
pub mod rdp;
pub mod render;
pub mod timer;
pub mod usb;
pub mod watch;
