#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use microbit::{
    display::nonblocking::{Display, GreyscaleImage},
    display_pins,
    hal::{
        gpio::p0::Parts as P0Parts,
        rtc::{Rtc, RtcInterrupt},
    },
    pac::{self, interrupt, RTC0, TIMER1},
};

#[cfg(feature = "v2")]
use microbit::hal::gpio::p1::Parts as P1Parts;

fn heart_image(inner_brightness: u8) -> GreyscaleImage {
    let b = inner_brightness;
    GreyscaleImage::new(&[
        [0, 7, 0, 7, 0],
        [7, b, 7, b, 7],
        [7, b, b, b, 7],
        [0, 7, b, 7, 0],
        [0, 0, 7, 0, 0],
    ])
}

// We use TIMER1 to drive the display, and RTC0 to update the animation.
// We set the TIMER1 interrupt to a higher priority than RTC0.

static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));
static ANIM_TIMER: Mutex<RefCell<Option<Rtc<RTC0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = pac::Peripherals::take() {
        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.reset();

        // RTC at 16Hz (32_768 / (2047 + 1))
        // 62.5ms period
        let mut rtc0 = Rtc::new(p.RTC0, 2047).unwrap();
        rtc0.enable_event(RtcInterrupt::Tick);
        rtc0.enable_interrupt(RtcInterrupt::Tick, None);
        rtc0.enable_counter();

        // Set up pins
        #[cfg(feature = "v1")]
        let pins = {
            let p0parts = P0Parts::new(p.GPIO);
            display_pins!(p0parts)
        };

        #[cfg(feature = "v2")]
        let pins = {
            let p0parts = P0Parts::new(p.P0);
            let p1parts = P1Parts::new(p.P1);
            display_pins!(p0parts, p1parts)
        };

        let display = Display::new(p.TIMER1, pins);

        cortex_m::interrupt::free(move |cs| {
            *DISPLAY.borrow(cs).borrow_mut() = Some(display);
            *ANIM_TIMER.borrow(cs).borrow_mut() = Some(rtc0);
        });
        if let Some(mut cp) = Peripherals::take() {
            unsafe {
                cp.NVIC.set_priority(pac::Interrupt::RTC0, 64);
                cp.NVIC.set_priority(pac::Interrupt::TIMER1, 128);
                pac::NVIC::unmask(pac::Interrupt::RTC0);
                pac::NVIC::unmask(pac::Interrupt::TIMER1);
            }
        }
    }

    loop {
        continue;
    }
}

#[interrupt]
fn TIMER1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    });
}

#[interrupt]
unsafe fn RTC0() {
    static mut STEP: u8 = 0;

    cortex_m::interrupt::free(|cs| {
        if let Some(rtc) = ANIM_TIMER.borrow(cs).borrow_mut().as_mut() {
            rtc.reset_event(RtcInterrupt::Tick);
        }
    });

    let inner_brightness = match *STEP {
        0..=8 => 9 - *STEP,
        9..=12 => 0,
        _ => unreachable!(),
    };

    cortex_m::interrupt::free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.show(&heart_image(inner_brightness));
        }
    });

    *STEP += 1;
    if *STEP == 13 {
        *STEP = 0
    };
}
