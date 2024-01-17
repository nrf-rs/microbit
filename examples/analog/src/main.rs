#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use microbit::{
    adc::{Adc, AdcConfig, Default},
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);
        let mut adc = Adc::new(board.ADC, AdcConfig::default_10bit());
        let mut anapin = board.edge.e00.into_floating_input(); // PAD0

        let numbers = [
            [
                [0, 0, 1, 0, 0],
                [0, 1, 0, 1, 0],
                [0, 1, 0, 1, 0],
                [0, 1, 0, 1, 0],
                [0, 0, 1, 0, 0],
            ],
            [
                [0, 0, 1, 0, 0],
                [0, 1, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ],
            [
                [0, 0, 1, 0, 0],
                [0, 1, 0, 1, 0],
                [0, 0, 1, 0, 0],
                [0, 1, 0, 0, 0],
                [0, 1, 1, 1, 0],
            ],
            [
                [0, 1, 1, 0, 0],
                [0, 0, 0, 1, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 0, 1, 0],
                [0, 1, 1, 0, 0],
            ],
            [
                [0, 1, 0, 0, 0],
                [1, 0, 0, 0, 0],
                [1, 0, 1, 0, 0],
                [1, 1, 1, 1, 0],
                [0, 0, 1, 0, 0],
            ],
        ];

        let sign_plus = [
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];

        #[cfg(feature = "v2")]
        #[allow(non_snake_case)]
        let letter_E = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        loop {
            let analog = adc.read_channel(&mut anapin);
            #[cfg(feature = "v2")]
            let Ok(analog) = analog
            else {
                display.show(&mut timer, letter_E, 10);
                continue;
            };
            let n_iter = numbers.iter();
            let mut count: usize = 0;
            for n_val in n_iter {
                if count == usize::from(i16::unsigned_abs(analog / 100)) {
                    display.show(&mut timer, *n_val, 10);
                    break;
                }
                count += 1;
            }
            if count == numbers.len() {
                display.show(&mut timer, sign_plus, 10);
            }
        }
    }
    panic!("End");
}
