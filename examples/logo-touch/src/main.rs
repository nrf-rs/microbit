#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_halt as _;

use embedded_hal::delay::DelayNs;
use microbit::{display::blocking::Display, hal::Timer, logo::Logo, Board};

// Shown on the LED matrix while the logo is being touched.
const SMILEY: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];

const BLANK: [[u8; 5]; 5] = [[0; 5]; 5];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut logo = Logo::new(board.pins.p1_04);

    // Remembering the previous state lets us derive "pressed" and "released"
    // edge events from the level reported by `is_touched`, the same way the
    // button examples derive edges from a button's level.
    let mut was_touched = false;

    defmt::info!("Touch the micro:bit logo!");
    loop {
        let touched = logo.is_touched(&mut timer);

        if touched && !was_touched {
            defmt::info!("pressed");
        } else if !touched && was_touched {
            defmt::info!("released");
        }
        was_touched = touched;

        if touched {
            display.show(&mut timer, SMILEY, 50);
        } else {
            display.show(&mut timer, BLANK, 50);
        }

        timer.delay_ms(20u32);
    }
}
