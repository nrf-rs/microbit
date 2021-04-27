#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use microbit::hal::{
    gpio::{p0, Level},
    prelude::*,
};

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        // Split GPIO pins
        #[cfg(feature = "v1")]
        let p0 = p0::Parts::new(p.GPIO);

        #[cfg(feature = "v2")]
        let p0 = p0::Parts::new(p.P0);

        // Set row of LED matrix to permanent high
        #[cfg(feature = "v1")]
        let _ = p0.p0_13.into_push_pull_output(Level::Low).set_high();

        #[cfg(feature = "v2")]
        let _ = p0.p0_21.into_push_pull_output(Level::Low).set_high();

        // Set 2 columns to output to control LED states
        #[cfg(feature = "v1")]
        let (mut led1, mut led2) = {
            (
                p0.p0_04.into_push_pull_output(Level::Low),
                p0.p0_06.into_push_pull_output(Level::Low),
            )
        };

        #[cfg(feature = "v2")]
        let (mut led1, mut led2) = {
            (
                p0.p0_28.into_push_pull_output(Level::Low),
                p0.p0_11.into_push_pull_output(Level::Low),
            )
        };

        // Configure button GPIOs as inputs
        #[cfg(feature = "v1")]
        let (button_a, button_b) = {
            (
                p0.p0_17.into_floating_input(),
                p0.p0_26.into_floating_input(),
            )
        };

        #[cfg(feature = "v2")]
        let (button_a, button_b) = {
            (
                p0.p0_14.into_floating_input(),
                p0.p0_23.into_floating_input(),
            )
        };

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
