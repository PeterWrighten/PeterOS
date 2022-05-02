#![no_std]

mod syscall;
use syscall::sys_yield;

pub fn yield_() -> isize {
    sys_yield()
}