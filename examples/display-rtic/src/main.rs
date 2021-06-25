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
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        clocks::Clocks,
        rtc::{Rtc, RtcInterrupt},
    },
    pac,
};

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

#[app(device = microbit::pac, peripherals = false)]
const APP: () = {
    struct Resources {
        display: Display<pac::TIMER1>,
        anim_timer: Rtc<pac::RTC0>,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = Board::take().unwrap();

        // Starting the low-frequency clock (needed for RTC to work)
        Clocks::new(board.CLOCK).start_lfclk();

        // RTC at 16Hz (32_768 / (2047 + 1))
        // 16Hz; 62.5ms period
        let mut rtc0 = Rtc::new(board.RTC0, 2047).unwrap();
        rtc0.enable_event(RtcInterrupt::Tick);
        rtc0.enable_interrupt(RtcInterrupt::Tick, None);
        rtc0.enable_counter();

        let display = Display::new(board.TIMER1, board.display_pins);

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
