#![no_std]
#![no_main]

#[no_mangle]
pub fn main() -> i32 {
    let args: Vec<_> = line.as_str().split(' ').collect();
    let mut args_copy: Vec<String> = args
        .iter()
        .map(|&arg| {
            let mut string = String::new();
            string.push_str(arg);
            string
        })
        .collect();

    args_copy
        .iter_mut()
        .for_each(|string| {
            string.push('\0');
        });
    let mut args_addr: Vec<*const u8> = args_copy
        .iter()
        .map(|arg| arg.as_ptr())
        .collect();
    args_addr.push(0 as *const u8);
    let mut input = String::new();
    if let Some((idx, _)) = args_copy
        .iter()
        .enumerate()
        .find(|(_, arg)| {
            arg.as_str() == "<\0"
        }) {
        input = args_copy[idx + 1].clone();
        args_copy.drain(idx..=idx+1);
    }

    let mut output = String::new();
    if let Some((idx, _)) = args_copy
        .iter()
        .enumerate()
        .find(|(_, arg)| {
            arg.as_str() == ">\0"
        }) {
        input = args_copy[idx + 1].clone();
        args_copy.drain(idx..=idx+1);
    }

    let pid = fork();
    if pid == 0 {
        // input redirection
        if !input.empty() {
            let input_fd = open(input.as_str(), OpenFlag::RDONLY);
            if input_fd == -1 {
                println!("Error when opening file {}", input);
                return -4;
            }
            let input_fd = input_fd as usize;
            close(0);
            assert_eq!(dup(input_fd), 0);
            close(input_fd);
        }

        if !output.empty() {
            let outout_fd = open(
                output.as_str(),
                OpenFlags::WRONLY | OpenFlags::CREATE,
            );

            if output_fd == -1 {
                println!("Error when opening file {}", output);
                return -4;
            }
            let output_fd = output_fd as usize;
            close(1);
            assert_eq!(dup(output_fd), 1);
            close(output_fd);
        }
        if exec(args_copy[0].as_ptr(), args_addr.as_slice()) == -1 {
            println!("Error when executing!");
            return -4;
        }
        unreachable!();
    } else {
        let mut exit_code: i32 = 0;
        let exit_pid = waitpid(pid as usize, &mut exit_code);
        assert_eq!(pid, exit_pid);
        println!("Shell: Process {} exited with code {}", pid, exit_code);
    }
    0
}