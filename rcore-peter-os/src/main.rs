// Move away standard library


#![no_std]

// Move away 'main()' function
#![no_main]

//Read Panic info
#![feature(panic_info_message)]

// Call lang_items mod to implement "panic handler"
#[macro_use]
mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {// extern means the call of extern Interface of other Programming language.
        // functional call Interface in RISC-V for C language.
        // inorder to read sbss and ebss's address.
        fn sbss(); 
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe {(a as *mut u8).write_volatile(0)}
    });
}



