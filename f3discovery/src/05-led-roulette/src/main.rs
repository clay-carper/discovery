#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::entry;

#[entry] // from cortex-m-rt crate
fn main() -> ! { // fn () -> doesn't return, ! in Rust is the never type (empty type in type theory)
    // not a traditional main function, this is just our entry point for the microcontroller, can name it anything we want to.
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}

// run with `cargo build --target thumbv7em-none-eabihf`
// Run GDB with `gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette`
