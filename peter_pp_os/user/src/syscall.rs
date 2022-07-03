const SYSCALL_PIPE: usize = 59;

/// function: open a pipe for current process
/// parameters: pipe: [usize; 2], contains fd of reader and writer
/// return: if error: -1; else 0. error: illegal address
/// syscall ID: 59
pub fn sys_pipe(pipe: *mut usize) -> isize {
    syscall(SYSCALL_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
}

