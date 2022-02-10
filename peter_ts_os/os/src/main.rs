#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use riscv::register::sie;

#[macro_use]
mod console;

mod sbi;
mod loader;
mod config;
mod task;
mod syscall;
mod timer;
mod trap;

global_asm!(include_str!("entry.asm"));
// global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, World!");
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}