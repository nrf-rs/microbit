#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{adc::AdcConfig, prelude::*, Adc, Timer},
};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);
        let mut adc: Adc = Adc::new(board.ADC, AdcConfig::default());
        let mut anapin = board.pins.p0_03.into_floating_input(); // PAD1

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

        #[allow(non_snake_case)]
        let letter_E = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        loop {
            let analog = adc.read(&mut anapin);
            match analog {
                Ok(v) => {
                    let n_iter = numbers.iter();
                    let mut count: usize = 0;
                    for n_val in n_iter {
                        if count == usize::from(i16::unsigned_abs(v / 100)) {
                            display.show(&mut timer, *n_val, 10);
                            break;
                        }
                        count += 1;
                    }
                    if count == numbers.len() {
                        display.show(&mut timer, sign_plus, 10);
                    }
                }
                Err(_e) => display.show(&mut timer, letter_E, 10),
            }
        }
    }
    panic!("End");
}
