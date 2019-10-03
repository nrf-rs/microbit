#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        /* Split GPIO pins */
        let gpio = p.GPIO.split();

        /* Set row of LED matrix to permanent high */
        gpio.pin13.into_push_pull_output().set_high();

        /* Set 2 columns to output to control LED states */
        let mut led1 = gpio.pin4.into_push_pull_output();
        let mut led2 = gpio.pin6.into_push_pull_output();

        /* Configure button GPIOs as inputs */
        let button_a = gpio.pin17.into_floating_input();
        let button_b = gpio.pin26.into_floating_input();

        loop {
            if button_a.is_high() {
                led1.set_high();
            } else {
                led1.set_low();
            }

            if button_b.is_high() {
                led2.set_high();
            } else {
                led2.set_low();
            }
        }
    }

    loop {
        continue;
    }
}
