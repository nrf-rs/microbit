//! An example of scrolling static text.
//!
//! It uses `TIMER1` to drive the display, and `RTC0` to animate the text.
#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use microbit::{
    board::Board,
    display::nonblocking::{Display, Frame, MicrobitFrame},
    hal::{
        clocks::Clocks,
        rtc::{Rtc, RtcInterrupt},
    },
    pac,
};
use microbit_text::scrolling::Animate;
use microbit_text::scrolling_text::ScrollingStaticText;
use rtic::app;

const MESSAGE: &[u8] = b"Hello, world!";

#[app(device = microbit::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        display: Display<pac::TIMER1>,
        anim_timer: Rtc<pac::RTC0>,
        scroller: ScrollingStaticText,
    }

    #[local]
    struct Local {
        frame: MicrobitFrame,
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

        let mut scroller = ScrollingStaticText::default();
        scroller.set_message(MESSAGE);

        (
            Shared {
                anim_timer: rtc0,
                display,
                scroller,
            },
            Local {
                frame: MicrobitFrame::default(),
            },
            init::Monotonics(),
        )
    }

    #[task(binds = TIMER1, priority = 2, shared = [display])]
    fn timer1(mut cx: timer1::Context) {
        cx.shared.display.lock(|d| d.handle_display_event());
    }

    #[task(binds = RTC0, priority = 1, shared = [anim_timer, display, scroller], local = [frame])]
    fn rtc0(mut cx: rtc0::Context) {
        cx.shared
            .anim_timer
            .lock(|t| t.reset_event(RtcInterrupt::Tick));
        if !cx.shared.scroller.lock(|s| s.is_finished()) {
            cx.shared.scroller.lock(|s| s.tick());
            let frame = cx.local.frame;
            cx.shared.scroller.lock(|s| frame.set(s));
            cx.shared.display.lock(|display| display.show_frame(frame));
        }
    }
}
