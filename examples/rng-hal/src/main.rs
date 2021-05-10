#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use microbit::{
    hal::rng,
    pac::{self, interrupt},
};

use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg32;

static RTC: Mutex<RefCell<Option<pac::RTC0>>> = Mutex::new(RefCell::new(None));
static RNG: Mutex<RefCell<Option<Pcg32>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {
            p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

            while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}

            /* And then set it back to 0 again, just because ?!? */
            p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

            defmt::info!("Welcome to the random number printer!");

            /* Use hardware RNG to initialise PRNG */
            let mut rng = rng::Rng::new(p.RNG);

            let mut seed: [u8; 16] = [0; 16];

            /* Read 4 bytes of data from hardware RNG */
            rng.random(&mut seed);

            let rng = Pcg32::from_seed(seed);
            *RNG.borrow(cs).borrow_mut() = Some(rng);

            p.RTC0.prescaler.write(|w| unsafe { w.bits(1) });
            p.RTC0.evtenset.write(|w| w.tick().set_bit());
            p.RTC0.intenset.write(|w| w.tick().set_bit());
            p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);

            unsafe {
                pac::NVIC::unmask(pac::Interrupt::RTC0);
            }
            pac::NVIC::unpend(pac::Interrupt::RTC0);
        });
    }

    loop {
        continue;
    }
}

// Define an exception, i.e. function to call when exception occurs. Here if our timer
// trips, we'll print out a random number to the serial port
#[interrupt]
fn RTC0() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let (Some(rtc), &mut Some(ref mut rng)) = (
            RTC.borrow(cs).borrow().as_ref(),
            RNG.borrow(cs).borrow_mut().deref_mut(),
        ) {
            defmt::info!("{:?}", rng.next_u32());
            rtc.events_tick.write(|w| unsafe { w.bits(0) });
        }
    });
}
