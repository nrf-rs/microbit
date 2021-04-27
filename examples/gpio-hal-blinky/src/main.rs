#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use microbit::hal::{
    gpio::{p0, Level},
    timer::Timer,
};

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        /* Split GPIO pins */
        #[cfg(feature = "v1")]
        let p0 = p0::Parts::new(p.GPIO);

        #[cfg(feature = "v2")]
        let p0 = p0::Parts::new(p.P0);

        let mut timer = Timer::new(p.TIMER0);

        #[cfg(feature = "v1")]
        let mut led = {
            let _ = p0.p0_04.into_push_pull_output(Level::Low);
            p0.p0_13.into_push_pull_output(Level::Low)
        };

        #[cfg(feature = "v2")]
        let mut led = {
            let _ = p0.p0_28.into_push_pull_output(Level::Low);
            p0.p0_21.into_push_pull_output(Level::Low)
        };

        loop {
            let _ = led.set_low();
            timer.delay_ms(1_000_u16);
            let _ = led.set_high();
            timer.delay_ms(1_000_u16);
        }
    }

    loop {
        continue;
    }
}
