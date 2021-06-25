#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use microbit::{
    hal::{clocks, rng, rtc},
    pac::{self, interrupt},
};

use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg32;

static RTC: Mutex<RefCell<Option<rtc::Rtc<pac::RTC0>>>> = Mutex::new(RefCell::new(None));
static RNG: Mutex<RefCell<Option<Pcg32>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut cp = pac::CorePeripherals::take().unwrap();
    let p = microbit::Peripherals::take().unwrap();

    cortex_m::interrupt::free(move |cs| {
        /* Start low frequency clock */
        clocks::Clocks::new(p.CLOCK).start_lfclk();

        defmt::info!("Welcome to the random number printer!");

        /* Use hardware RNG to initialise PRNG */
        let mut rng = rng::Rng::new(p.RNG);

        let mut seed: [u8; 16] = [0; 16];

        /* Read 4 bytes of data from hardware RNG */
        rng.random(&mut seed);

        let rng = Pcg32::from_seed(seed);
        *RNG.borrow(cs).borrow_mut() = Some(rng);

        let mut rtc = rtc::Rtc::new(p.RTC0, 1).unwrap();
        rtc.enable_counter();
        rtc.enable_interrupt(rtc::RtcInterrupt::Tick, Some(&mut cp.NVIC));
        rtc.enable_event(rtc::RtcInterrupt::Tick);

        *RTC.borrow(cs).borrow_mut() = Some(rtc);

        pac::NVIC::unpend(pac::Interrupt::RTC0);
    });

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
            rtc.reset_event(rtc::RtcInterrupt::Tick);
        }
    });
}
