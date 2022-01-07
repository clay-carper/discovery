#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        ptr::read_volatile(0x4800_1800 as *const u32);
    }

    loop {}
}