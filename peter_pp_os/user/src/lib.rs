mod syscall;
use syscall::{sys_pipe, sys_exec};

pub fn pipe(pipe_fd: &mut [usize]) -> isize {
    sys_pipe(pipe_fd.as_ptr() as usize as *mut _)
}

pub fn exec(path: &str, args: &[*const u8]) -> isize {
    sys_exec(path, args)
}
