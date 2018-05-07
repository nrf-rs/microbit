#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate microbit;
extern crate panic_abort;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;

fn main() {
    if let Some(p) = microbit::Peripherals::take() {
        let mut gpio = p.GPIO.split();
        let mut delay = Delay::new(p.TIMER0);
        let mut led = gpio.pin13.into_push_pull_output();
        let _ = gpio.pin4.into_push_pull_output();

        loop {
            led.set_low();
            delay.delay_ms(1_000_u16);
            led.set_high();
            delay.delay_ms(1_000_u16);
        }
    }
}
