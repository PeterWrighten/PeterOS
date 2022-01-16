// Move away standard library
#![no_std]

// Move away 'main()' function
#![no_main]

// Invoke lang_items mod to implement "panic handler"
mod lang_items;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));




