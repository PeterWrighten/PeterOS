// 'lang_items' mod, without std support.

// use PanicInfo to rebuild "panic handler"
use core::panic::PanicInfo;

use crate::sbi::shutdown;

// &PanicInfo is message of Panic from compiler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
