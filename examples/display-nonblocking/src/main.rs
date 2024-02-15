#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{interrupt, interrupt::InterruptExt, interrupt::Priority, peripherals::TIMER1},
    time::{Duration, Ticker},
};

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

// We use TIMER1 to drive the display, and RTC ticker to update the animation.
// We set the TIMER1 interrupt to a higher priority than thread mode.

static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let board = Board::default();

    // Create display
    let display = Display::new(board.TIMER1, board.display_pins);

    cortex_m::interrupt::free(move |cs| {
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
    });
    interrupt::TIMER1.set_priority(Priority::P0);

    let mut ticker = Ticker::every(Duration::from_micros(65000));
    let mut step: u8 = 0;
    loop {
        ticker.next_blocking();
        let inner_brightness = match step {
            0..=8 => 9 - step,
            9..=12 => 0,
            _ => unreachable!(),
        };

        cortex_m::interrupt::free(|cs| {
            if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                display.show(&heart_image(inner_brightness));
            }
        });

        step += 1;
        if step == 13 {
            step = 0
        };
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
