#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod lang_items;
mod console;
mod batch;
mod sync;
mod sbi;
mod trap;


global_asm!(include_str!("link_app.S"));
