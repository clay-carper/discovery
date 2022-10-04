#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // Send a single character (one byte) by writing to the `TDR` register.
    // This causes the `USART` peripheral to send one byte of information through the serial interface
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        usart1
            .tdr
            .write(|w| w.tdr().bits(u16::from(*byte)) );
    }
    

    loop {}
}
