#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;


#[macro_use]
mod console;
mod batch;
mod lang_items;
mod sync;
mod sbi;
mod trap;
mod syscall;

global_asm!(include_str!("link_app.S"));
global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}