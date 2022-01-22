#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Test store_fault.");
    println!("Kernel should kill this app!");
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    }
    0
}