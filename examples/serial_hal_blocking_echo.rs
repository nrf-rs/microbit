#![no_main]
#![no_std]

use panic_halt as _;

use microbit::hal::prelude::*;
use microbit::hal::serial::BAUD115200;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        /* Split GPIO pins */
        let gpio = p.GPIO.split();

        /* Initialise serial port on the micro:bit */
        let (mut tx, mut rx) = microbit::serial_port(gpio, p.UART0, BAUD115200);

        /* Print a nice hello message */
        let s = b"Please type characters to echo:\r\n";

        let _ = s.iter().map(|c| nb::block!(tx.write(*c))).last();

        /* Endless loop */
        loop {
            /* Read and echo back */
            if let Ok(c) = nb::block!(rx.read()) {
                let _ = nb::block!(tx.write(c));
            }
        }
    }

    loop {
        continue;
    }
}
