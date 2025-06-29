#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use core::arch::global_asm;
use core::mem::MaybeUninit;

use my_println::println;

global_asm!(include_str!("../src/start.s"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() {
    let mut vec: Vec<MaybeUninit<u32>> = Vec::new();
    for i in 0..10 {
        vec.push(MaybeUninit::new(i));
    }
    println!("Allocated vector with {} elements", vec.len());
}
