// 'lang_items' mod, without std support.

// use PanicInfo to rebuild "panic handler"
use core::panic::PanicInfo;


// &PanicInfo is message of Panic from compiler.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}