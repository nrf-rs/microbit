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

#[app(device = microbit::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        display: Display<pac::TIMER1>,
        anim_timer: Rtc<pac::RTC0>,
    }

    #[local]
    struct Local {
        step: u8,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board = Board::new(cx.device, cx.core);

        // Starting the low-frequency clock (needed for RTC to work)
        Clocks::new(board.CLOCK).start_lfclk();

        // RTC at 16Hz (32_768 / (2047 + 1))
        // 16Hz; 62.5ms period
        let mut rtc0 = Rtc::new(board.RTC0, 2047).unwrap();
        rtc0.enable_event(RtcInterrupt::Tick);
        rtc0.enable_interrupt(RtcInterrupt::Tick, None);
        rtc0.enable_counter();

        let display = Display::new(board.TIMER1, board.display_pins);

        (
            Shared {
                anim_timer: rtc0,
                display,
            },
            Local { step: 0 },
            init::Monotonics(),
        )
    }

    #[task(binds = TIMER1, priority = 2, shared = [display])]
    fn timer1(mut cx: timer1::Context) {
        cx.shared.display.lock(|d| d.handle_display_event());
    }

    #[task(binds = RTC0, priority = 1, shared = [anim_timer, display], local = [step])]
    fn rtc0(mut cx: rtc0::Context) {
        cx.shared
            .anim_timer
            .lock(|t| t.reset_event(RtcInterrupt::Tick));

        let inner_brightness = match cx.local.step {
            0..=8 => 9 - *cx.local.step,
            9..=12 => 0,
            _ => unreachable!(),
        };

        cx.shared.display.lock(|display| {
            display.show(&heart_image(inner_brightness));
        });

        *cx.local.step += 1;
        if *cx.local.step == 13 {
            *cx.local.step = 0;
        };
    }
}
