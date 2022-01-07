#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // enable the GPIOE peripheral
    rcc.ahbenr.write(|w| w.iopeen().set_bit());

    // configure the pins as outputs
    gpioe.moder.write(|w| {
        w.moder8().output();  // LD3 | PE8 Blue, NW light
        w.moder9().output();  // LD4 | PE9 Red N light
        w.moder10().output(); // LD5 | PE10 Orange NE light
        w.moder11().output(); // LD7 | PE11 Green E light 
        w.moder12().output(); // LD9 | PE12 Blue SE light
        w.moder13().output(); // LD10 | PE13 Red S light
        w.moder14().output(); // LD8 | PE14 Orange SW light
        w.moder15().output() // LD6 | PE15 Green W light
    });

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    aux8::bkpt();

    loop {}
}
