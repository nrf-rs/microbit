#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use microbit::hal::{self, gpio::Level, prelude::*};

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        // Split GPIO pins
        let gpio = hal::gpio::p0::Parts::new(p.GPIO);

        // Set row of LED matrix to permanent high
        let _ = gpio.p0_13.into_push_pull_output(Level::Low).set_high();

        // Set 2 columns to output to control LED states
        let mut led1 = gpio.p0_04.into_push_pull_output(Level::Low);
        let mut led2 = gpio.p0_06.into_push_pull_output(Level::Low);

        // Configure button GPIOs as inputs
        let button_a = gpio.p0_17.into_floating_input();
        let button_b = gpio.p0_26.into_floating_input();

        loop {
            if let Ok(true) = button_a.is_high() {
                let _ = led1.set_high();
            } else {
                let _ = led1.set_low();
            }

            if let Ok(true) = button_b.is_high() {
                let _ = led2.set_high();
            } else {
                let _ = led2.set_low();
            }
        }
    }

    loop {
        continue;
    }
}
