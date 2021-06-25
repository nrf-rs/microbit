#![no_main]
#![no_std]

use panic_halt as _;

use core::fmt::Write;
use microbit::hal;
use microbit::hal::prelude::*;
use microbit::hal::uart::Baudrate;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let gpio = hal::gpio::p0::Parts::new(p.GPIO);

        /* Initialise serial port on the micro:bit */
        let mut serial = microbit::serial_port!(gpio, p.UART0, Baudrate::BAUD115200);

        /* Print a nice hello message */
        write!(serial, "Please type characters to echo:\r\n");

        /* Endless loop */
        loop {
            /* Read and echo back */
            if let Ok(c) = nb::block!(serial.read()) {
                let _ = nb::block!(serial.write(c));
            }
        }
    }

    loop {
        continue;
    }
}
