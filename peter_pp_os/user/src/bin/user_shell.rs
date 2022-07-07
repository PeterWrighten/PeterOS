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
    0
}