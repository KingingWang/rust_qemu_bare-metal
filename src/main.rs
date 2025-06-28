#![no_std]
#![no_main]
pub mod uart_console;
use core::arch::global_asm;
mod panic;

global_asm!(include_str!("start.s"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() {
    loop {}
}
