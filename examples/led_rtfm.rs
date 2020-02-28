//! A complete working example.
//!
//! This requires `cortex-m-rtfm` v0.5.
//!
//! It uses `TIMER1` to drive the display, and `RTC0` to update a simple
//! animated image.
#![no_main]
#![no_std]

use panic_halt as _;

use microbit::display::image::GreyscaleImage;
use microbit::display::{self, Display, Frame, MicrobitDisplayTimer, MicrobitFrame};
use microbit::hal::lo_res_timer::{LoResTimer, FREQ_16HZ};
use microbit::hal::nrf51;
use rtfm::app;

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

#[app(device = microbit::hal::nrf51, peripherals = true)]
const APP: () = {
    struct Resources {
        gpio: nrf51::GPIO,
        display_timer: MicrobitDisplayTimer<nrf51::TIMER1>,
        anim_timer: LoResTimer<nrf51::RTC0>,
        display: Display<MicrobitFrame>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mut p: nrf51::Peripherals = cx.device;

        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.reset();

        let mut rtc0 = LoResTimer::new(p.RTC0);
        // 16Hz; 62.5ms period
        rtc0.set_frequency(FREQ_16HZ);
        rtc0.enable_tick_event();
        rtc0.enable_tick_interrupt();
        rtc0.start();

        let mut timer = MicrobitDisplayTimer::new(p.TIMER1);
        display::initialise_display(&mut timer, &mut p.GPIO);

        init::LateResources {
            gpio: p.GPIO,
            display_timer: timer,
            anim_timer: rtc0,
            display: Display::new(),
        }
    }

    #[task(binds = TIMER1, priority = 2,
           resources = [display_timer, gpio, display])]
    fn timer1(mut cx: timer1::Context) {
        display::handle_display_event(
            &mut cx.resources.display,
            cx.resources.display_timer,
            cx.resources.gpio,
        );
    }

    #[task(binds = RTC0, priority = 1,
           resources = [anim_timer, display])]
    fn rtc0(mut cx: rtc0::Context) {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        static mut STEP: u8 = 0;

        &cx.resources.anim_timer.clear_tick_event();

        let inner_brightness = match *STEP {
            0..=8 => 9 - *STEP,
            9..=12 => 0,
            _ => unreachable!(),
        };

        FRAME.set(&mut heart_image(inner_brightness));
        cx.resources.display.lock(|display| {
            display.set_frame(FRAME);
        });

        *STEP += 1;
        if *STEP == 13 {
            *STEP = 0
        };
    }
};
