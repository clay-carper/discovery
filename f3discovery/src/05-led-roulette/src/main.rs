#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry] // from cortex-m-rt crate
fn main() -> ! { // fn () -> doesn't return, ! in Rust is the never type (empty type in type theory)
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 500_u16;

    loop {
        leds[0].on().ok();
        delay.delay_ms(half_period);

        leds[0].off().ok();
        delay.delay_ms(half_period);
    }
}

// run with `cargo build --target thumbv7em-none-eabihf`
// Run GDB with `gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette`
