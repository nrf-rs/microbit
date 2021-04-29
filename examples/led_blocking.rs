#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

use microbit::hal::{gpio::p0::Parts as P0Parts, prelude::*, Timer};

#[cfg(feature = "microbit-v1")]
use microbit::display_pins_v1 as display_pins;

#[cfg(feature = "microbit-v2")]
use microbit::{display_pins_v2 as display_pins, hal::gpio::p1::Parts as P1Parts};

use microbit::led;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::pac::Peripherals::take() {
        let mut timer = Timer::new(p.TIMER0);

        // Set up pins
        #[cfg(feature = "microbit-v1")]
        let pins = {
            let p0parts = P0Parts::new(p.GPIO);
            display_pins!(p0parts)
        };

        #[cfg(feature = "microbit-v2")]
        let pins = {
            let p0parts = P0Parts::new(p.P0);
            let p1parts = P1Parts::new(p.P1);
            display_pins!(p0parts, p1parts)
        };

        // Display
        let mut leds = led::Display::new(pins);

        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        let heart = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_R = [
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_u = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_s = [
            [0, 0, 0, 0, 0],
            [0, 0, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_t = [
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];
        loop {
            leds.display(&mut timer, letter_I, 1000);
            leds.display(&mut timer, heart, 1000);
            leds.display(&mut timer, letter_R, 1000);
            leds.display(&mut timer, letter_u, 1000);
            leds.display(&mut timer, letter_s, 1000);
            leds.display(&mut timer, letter_t, 1000);
            leds.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
