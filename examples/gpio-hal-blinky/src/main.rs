#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board::Board, hal::timer::Timer};

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    let _ = board.display_pins.col1.set_low();
    let mut row1 = board.display_pins.row1;

    loop {
        let _ = row1.set_low();
        timer.delay_ms(1_000);
        let _ = row1.set_high();
        timer.delay_ms(1_000);
    }
}
