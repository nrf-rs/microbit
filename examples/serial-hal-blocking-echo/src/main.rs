#![no_main]
#![no_std]

use panic_halt as _;

use core::fmt::Write;
use microbit::hal::prelude::*;

#[cfg(feature = "v1")]
use microbit::{
    hal::uart,
    hal::uart::{Baudrate, Parity},
};
#[cfg(feature = "v2")]
use microbit::{
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    /* Print a nice hello message */
    write!(serial, "Please type characters to echo:\r\n").unwrap();

    /* Endless loop */
    loop {
        /* Read and echo back */
        if let Ok(c) = nb::block!(serial.read()) {
            let _ = nb::block!(serial.write(c));
        }
    }
}
