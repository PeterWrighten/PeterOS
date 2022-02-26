#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
mod console;

#[macro_use]
extern crate bitflags;

mod lang_items;
mod sync;
mod loader;
mod sbi;
mod timer;
mod mm;
mod config;
mod task;
mod syscall;
mod trap;


