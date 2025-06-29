use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    println!("PANIC: {}", _info);
    loop {}
}
