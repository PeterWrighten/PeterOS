const SYSCALL_PIPE: usize = 59;
const SYSCALL_DUP: usize = 24;

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

/// function: duplicate an opened file into a newly allocated fd
/// para: fd
/// return: error: -1; else: fd
/// possible error reason: input fd is not corresponded with a legal file
/// syscall ID: 24
pub fn sys_dup(fd: usize) -> isize {
    syscall(SYSCALL_DUP, [fd, 0, 0])
}