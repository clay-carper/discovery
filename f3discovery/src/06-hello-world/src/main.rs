#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux6::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {

    panic!("Hello, world!");

}

// Make sure the discovery board has the correct pins shorted (SWO <-> PB3) OR the SB10 solder bridge is connected
// Run itmdump with the following:
// `touch itm.txt`
// `itmdump -F -f itm.txt
// Make damn sure that `OpenOCD` and `itmdump` are running in the same directory (probably /tmp)