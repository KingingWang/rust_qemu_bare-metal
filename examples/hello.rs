#![no_std]
#![no_main]

use core::arch::global_asm;
use my_println::println;

global_asm!(include_str!("../src/start.s"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() {
    println!("Hello, world!");
}
