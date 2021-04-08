#![no_std]
#![allow(non_camel_case_types)]

pub use nrf51_hal as hal;

pub use nb::*;

pub use crate::hal::nrf51::*;

pub mod display;
pub mod led;

#[macro_export]
macro_rules! serial_port {
    ( $gpio:expr, $uart:expr, $speed:expr ) => {{
        use nrf51_hal::serial::Serial;

        /* Configure RX and TX pins accordingly */
        let tx = $gpio.pin24.into_push_pull_output().into();
        let rx = $gpio.pin25.into_floating_input().into();

        /* Set up serial port using the prepared pins */
        let serial = Serial::uart0($uart, tx, rx, $speed);
        serial.split()
    }};
}
