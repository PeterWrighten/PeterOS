// Move away standard library
#![no_std]

// Move away 'main()' function
#![no_main]

// Invoke lang_items mod to implement "panic handler"
mod lang_items;
mod sbi;

#[macro_use]
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello world!");
    loop{}
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



