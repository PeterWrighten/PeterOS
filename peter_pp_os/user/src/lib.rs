mod syscall;
use syscall::{sys_pipe, sys_exec};

pub fn pipe(pipe_fd: &mut [usize]) -> isize {
    sys_pipe(pipe_fd.as_ptr() as usize as *mut _)
}

pub fn exec(path: &str, args: &[*const u8]) -> isize {
    sys_exec(path, args)
}


/// argc: counter
/// argv: arg_base address
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(argc: usize, argv: usize) -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let mut v : Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start = unsafe {
            ((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile()
        };
        let len = (0usize..).find(|i| unsafe {
            ((str_start + *i) as *const u8).read_volatile() == 0
        }).unwrap();
        v.push(
            core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(str_start as *const u8, len)
            }).unwrap()
        );
    }
    exit(main(argc, v.as_slice()));
}

pub fn kill(pid: usize, signal: i32) -> isize {
    sys_kill(pid, signal)
}

pub fn sigaction(
    signum: i32,
    action: *const SignalAction,
    old_action: *const SignalAction
) -> isize {
    sys_sigaction(signum, action, old_action)
}

pub fn sigprocmask(mask: u32) -> isize {
    sys_sigprocmask(mask)
}

pub fn sigreturn() -> isize {
    sys_sigreturn()
}


