#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Busy waiting

    // Set the timer to go off in `ms` ticks
    // 1 tick = 1ms
    tim6.arr.write(|w| w.arr().bits(ms));

    // The CEN register can be used to enable to the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    // Wait until the alarm goes of (the `UIF` bit of the status register (`SR`))
    while !tim6.sr.read().uif().bit_is_set() {} // Waiting until some condition is met (i.e. `UIF` becomes `1`), is called busy waiting

    // CLear teh update event flag (set `UIF` to `0`)
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // Timer initialization

    // Power the `TIM6` bit to 1, in the APB1ENR register of the RCC register block
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());

    // Configuration, we want the timer in OPM Select pulse mode
    // CEN Keeps the counter disabled, for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    // We'll have the timer operate at a frequency of 1 KHz to match the 1 millisecond unitary value of our delay function
    // define pcs (prescaler) for the counter (1kHz = 8MHz / (psc + 1)) <=> (1 kHz = 8000 kHz / (psc + 1)) => psc = 7999
    tim6.psc.write(|w| w.psc().bits(7_999));

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
