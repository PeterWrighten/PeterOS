use crate::syscall::sys_read;
use exec::sys_exec;

mod syscall;
mod exec;
pub mod console;


pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

pub fn exec(path: &str) -> isize {
    sys_exec(path)
}

pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => { yield_(); }
            exit_pid => {
                return exit_pid;
            }
        }
    }
}