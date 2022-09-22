#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry] // from cortex-m-rt crate
fn main() -> ! { // fn () -> doesn't return, ! in Rust is the never type (empty type in type theory)
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50_u16;

    loop {

        for index in 0..8 { // make a new variable to index over
            let next = (index + 1) % 8; // increment the index, and pass it to a new var to address the arrays
                                               // Note: my initial attempt I skipped this step, but due to the difference
                                               //       in typing and how the value is passed to leds, it won't work thatta way
            
            leds[next].on().ok();
            delay.delay_ms(half_period);

            leds[next].off().ok();
            delay.delay_ms(half_period);
        }
        
    }
}

// run with `cargo build --target thumbv7em-none-eabihf`
// Run GDB with `gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette`
