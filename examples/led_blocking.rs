#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

use microbit::hal::{
    gpio::{p0::Parts as P0Parts, Level},
    prelude::*,
    Timer,
};

use microbit::led;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::pac::Peripherals::take() {
        let mut timer = Timer::new(p.TIMER0);
        let p0parts = P0Parts::new(p.GPIO);

        // Display
        let pins = led::Pins {
            row1: p0parts.p0_13.into_push_pull_output(Level::Low),
            row2: p0parts.p0_14.into_push_pull_output(Level::Low),
            row3: p0parts.p0_15.into_push_pull_output(Level::Low),
            col1: p0parts.p0_04.into_push_pull_output(Level::Low),
            col2: p0parts.p0_05.into_push_pull_output(Level::Low),
            col3: p0parts.p0_06.into_push_pull_output(Level::Low),
            col4: p0parts.p0_07.into_push_pull_output(Level::Low),
            col5: p0parts.p0_08.into_push_pull_output(Level::Low),
            col6: p0parts.p0_09.into_push_pull_output(Level::Low),
            col7: p0parts.p0_10.into_push_pull_output(Level::Low),
            col8: p0parts.p0_11.into_push_pull_output(Level::Low),
            col9: p0parts.p0_12.into_push_pull_output(Level::Low),
        };
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
