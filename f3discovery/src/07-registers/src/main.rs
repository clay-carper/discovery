#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprintln, ITM};

// Print the current contents of odr
fn iprint_odr(itm: &mut ITM) {
    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }
}

#[entry]
fn main() -> ! {
    let mut itm = aux7::init().0;

    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Print the initial contents of ORD
        iprint_odr(&mut itm);

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9); // Writing 1 << 9 (`BS9 = 1`) to `BSRR`  sets `PE9 high`; turns on the North LED on
        iprint_odr(&mut itm);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11); // Writing 1 << 11 (`BS11 = 1`) sets `PE11 high`; turns the East LED on
        iprint_odr(&mut itm);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16)); // Writing 1 << 25 (`BS9 = 1`) to `BSRR`  sets `PE9 lwo`; turns on the North LED off
        iprint_odr(&mut itm);

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16)); // Writing 1 << 27 (`BS11 = 1`) sets `PE11 high`; turns the East LED off
        iprint_odr(&mut itm);
    }

    loop {}
}
