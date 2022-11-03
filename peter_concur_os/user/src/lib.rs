pub fn thread_create(entry: usize, arg: usize) -> isize {
    sys_thread_create(entry, arg)
}

pub fn waittid(tid: usize) -> isize {
    loop {
        match sys_waittid(tid) {
            // -2 means waiting thread exists but not exit
            -2 => { yield_(); }
            exit_code => return exit_code,
        }
    }
}