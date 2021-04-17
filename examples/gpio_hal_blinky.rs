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
        let gpio = p0::Parts::new(p.GPIO);

        let mut timer = Timer::new(p.TIMER0);
        let mut led = gpio.p0_13.into_push_pull_output(Level::Low);
        let _ = gpio.p0_04.into_push_pull_output(Level::Low);

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
