#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_halt;

use microbit::hal::delay::{Timer, Generic, TIMER0};
use microbit::hal::prelude::*;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let mut gpio = p.GPIO.split();
        let mut delay = Timer::<Generic, TIMER0>::new(p.TIMER0, 4).into_delay();
        let mut led = gpio.pin13.into_push_pull_output();
        let _ = gpio.pin4.into_push_pull_output();

        loop {
            led.set_low();
            delay.delay_ms(1_000_u16);
            led.set_high();
            delay.delay_ms(1_000_u16);
        }
    }

    loop {
        continue;
    }
}
