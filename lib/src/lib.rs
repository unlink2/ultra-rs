#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(naked_functions)]

extern crate embedgdb;

pub mod color;
pub mod font;
pub mod input;
pub mod interrupt;
pub mod math;
pub mod memory;
pub mod menu;
pub mod monitor;
pub mod rdp;
pub mod render;
pub mod usb;