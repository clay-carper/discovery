#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9); // Writing 1 << 9 (`BS9 = 1`) to `BSRR`  sets `PE9 high`; turns on the North LED on

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11); // Writing 1 << 11 (`BS11 = 1`) sets `PE11 high`; turns the East LED on

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16)); // Writing 1 << 25 (`BS9 = 1`) to `BSRR`  sets `PE9 lwo`; turns on the North LED off

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16)); // Writing 1 << 27 (`BS11 = 1`) sets `PE11 high`; turns the East LED off
    }

    loop {}
}
