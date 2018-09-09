#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_abort;

#[macro_use(block)]
extern crate microbit;

use microbit::hal::prelude::*;
use microbit::hal::serial::BAUD115200;

use cortex_m_rt::entry;

#[entry]
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
