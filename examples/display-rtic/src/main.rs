//! A complete working example.
//!
//! This requires `cortex-m-rtfm` v0.5.
//!
//! It uses `TIMER1` to drive the display, and `RTC0` to update a simple
//! animated image.
#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use microbit::{
    display::nonblocking::{Display, GreyscaleImage},
    display_pins,
    hal::{
        gpio::p0::Parts as P0Parts,
        rtc::{Rtc, RtcInterrupt},
    },
    pac,
};

#[cfg(feature = "v2")]
use microbit::hal::gpio::p1::Parts as P1Parts;

use rtic::app;

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

#[app(device = microbit::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        display: Display<pac::TIMER1>,
        anim_timer: Rtc<pac::RTC0>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let p: pac::Peripherals = cx.device;

        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.reset();

        // RTC at 16Hz (32_768 / (2047 + 1))
        // 16Hz; 62.5ms period
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

        init::LateResources {
            anim_timer: rtc0,
            display,
        }
    }

    #[task(binds = TIMER1, priority = 2, resources = [display])]
    fn timer1(cx: timer1::Context) {
        cx.resources.display.handle_display_event();
    }

    #[task(binds = RTC0, priority = 1, resources = [anim_timer, display])]
    fn rtc0(mut cx: rtc0::Context) {
        static mut STEP: u8 = 0;

        cx.resources.anim_timer.reset_event(RtcInterrupt::Tick);

        let inner_brightness = match *STEP {
            0..=8 => 9 - *STEP,
            9..=12 => 0,
            _ => unreachable!(),
        };

        cx.resources.display.lock(|display| {
            display.show(&heart_image(inner_brightness));
        });

        *STEP += 1;
        if *STEP == 13 {
            *STEP = 0
        };
    }
};
