#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;

use microbit::pac::{self, interrupt};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

static RNG: Mutex<RefCell<Option<pac::RNG>>> = Mutex::new(RefCell::new(None));
static RTC: Mutex<RefCell<Option<pac::RTC0>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}

        /* And then set it back to 0 again, just because ?!? */
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

        defmt::info!("Welcome to the random number printer!");

        p.RTC0.prescaler.write(|w| unsafe { w.bits(1) });
        p.RTC0.evtenset.write(|w| w.tick().set_bit());
        p.RTC0.intenset.write(|w| w.tick().set_bit());
        p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

        /* Enable error correction for better values */
        p.RNG.config.write(|w| w.dercen().enabled());

        /* Enable random number generation */
        p.RNG.tasks_start.write(|w| unsafe { w.bits(1) });

        cortex_m::interrupt::free(move |cs| {
            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);
            *RNG.borrow(cs).borrow_mut() = Some(p.RNG);
        });

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::RTC0);
        }
        pac::NVIC::unpend(pac::Interrupt::RTC0);
    }

    loop {
        continue;
    }
}

// Define an exception, i.e. function to call when exception occurs. Here if our timer
// trips, we'll print some random number
#[interrupt]
fn RTC0() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let Some(rtc) = RTC.borrow(cs).borrow().as_ref() {
            let count = if let Some(rng) = RNG.borrow(cs).borrow().as_ref() {
                /* Let's wait until we have a new random value */
                while rng.events_valrdy.read().bits() == 0 {}

                let num = rng.value.read().bits();

                /* Clear event for next random number value */
                rng.events_valrdy.write(|w| unsafe { w.bits(0) });

                num
            } else {
                0
            };

            defmt::info!("{:?}", count);
            rtc.events_tick.write(|w| unsafe { w.bits(0) });
        }
    });
}
