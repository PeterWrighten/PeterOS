const SYSCALL_PIPE: usize = 59;

/// function: open a pipe for current process
/// parameters: pipe: [usize; 2], contains fd of reader and writer
/// return: if error: -1; else 0. error: illegal address
/// syscall ID: 59
pub fn sys_pipe(pipe: *mut usize) -> isize {
    syscall(SYSCALL_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
}

pub fn sys_exec(path: &str, args: &[*const u8]) -> isize {
    syscall(SYSCALL_EXEC, [path.as_ptr() as usize, args.as_ptr() as usize, 0])
}