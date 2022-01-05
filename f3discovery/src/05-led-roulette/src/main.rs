#![deny(unsafe_code)]
#![no_main]
#![no_std]

//use volatile::Volatile;
use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! { // This type indicates the function can't return i.e. it will never terminate
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50_u16;
    //let v_half_period = Volatile::new(&mut half_period);

    loop {
        for i in 0..8 {
            leds[i].on().ok();
            delay.delay_ms(half_period);
            leds[i].off().ok();
            delay.delay_ms(half_period/10); 
        }
        
    }
}
