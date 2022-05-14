use crate::syscall::sys_read;

mod syscall;
mod exec;
mod console;

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}