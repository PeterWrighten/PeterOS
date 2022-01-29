#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;

mod sbi;

global_asm!(include_str!("entry.asm"));
// global_asm!(include_str!("link_app.S"));