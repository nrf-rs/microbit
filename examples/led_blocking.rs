#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_abort;

#[macro_use(entry, exception)]
extern crate microbit;

use core::fmt::Write;

use rt::ExceptionFrame;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;

use microbit::led;

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

entry!(main);
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let mut gpio = p.GPIO.split();
        let mut delay = Delay::new(p.TIMER0);

        // Display
        let row1 = gpio.pin13.into_push_pull_output();
        let row2 = gpio.pin14.into_push_pull_output();
        let row3 = gpio.pin15.into_push_pull_output();
        let col1 = gpio.pin4.into_push_pull_output();
        let col2 = gpio.pin5.into_push_pull_output();
        let col3 = gpio.pin6.into_push_pull_output();
        let col4 = gpio.pin7.into_push_pull_output();
        let col5 = gpio.pin8.into_push_pull_output();
        let col6 = gpio.pin9.into_push_pull_output();
        let col7 = gpio.pin10.into_push_pull_output();
        let col8 = gpio.pin11.into_push_pull_output();
        let col9 = gpio.pin12.into_push_pull_output();
        let mut leds = led::Display::new(
            col1, col2, col3,
            col4, col5, col6,
            col7, col8, col9,
            row1, row2, row3,
        );

        // Configure RX and TX pins accordingly
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

        let _ = write!(tx, "\n\rStarting!\n\r");

        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        let heart = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_U = [
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        let bright_image1 = [
            [0, 2, 4, 6, 8],
            [2, 4, 6, 8, 10],
            [4, 6, 8, 10, 12],
            [6, 8, 10, 12, 14],
            [8, 10, 12, 14, 16],
        ];

        let bright_image2 = [
            [8, 4, 3, 2, 1],
            [13, 8, 4, 3, 2],
            [14, 13, 8, 4, 3],
            [15, 15, 13, 8, 4],
            [16, 15, 14, 13, 8],
        ];

        loop {
            write!(tx, "I <3 Rust on the micro:bit!\n\r");
            leds.display(&mut delay, letter_I, 1000);
            leds.display(&mut delay, heart, 1000);
            leds.display(&mut delay, letter_U, 1000);
            leds.clear();
            delay.delay_ms(250_u32);
            leds.display_bright(&mut delay, bright_image1, 1000);
            leds.display_bright(&mut delay, bright_image2, 1000);
            leds.clear();
            delay.delay_ms(250_u32);
        }
    }

    panic!("End");
}
