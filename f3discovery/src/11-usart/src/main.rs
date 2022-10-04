#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // Send a single character (one byte) by writing to the `TDR` register.
    // This causes the `USART` peripheral to send one byte of information through the serial interface
    // Works as intended when using `next` to step through. However, both `continue` and running in 
    // release mode give a jumbled string. Less than ideal, for sure.
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        usart1
            .tdr
            .write(|w| w.tdr().bits(u16::from(*byte)) );
    }
    

    loop {}
}
