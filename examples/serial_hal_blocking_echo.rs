#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt;

use cortex_m_rt::ExceptionFrame;

extern crate panic_abort;

#[macro_use]
extern crate microbit;

use microbit::hal::prelude::*;
use microbit::hal::serial::BAUD115200;

exception!(*, default_handler);

fn default_handler(_irqn: i16) {}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}
entry!(main);

fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        /* Split GPIO pins */
        let mut gpio = p.GPIO.split();

        /* Initialise serial port on the micro:bit */
        let (mut tx, mut rx) = microbit::serial_port(gpio, p.UART0, BAUD115200);

        /* Print a nice hello message */
        let s = b"Please type characters to echo:\r\n";

        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();

        /* Endless loop */
        loop {
            /* Read and echo back */
            if let Ok(c) = block!(rx.read()) {
                let _ = block!(tx.write(c));
            }
        }
    }

    loop {}
}
