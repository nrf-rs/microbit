#![no_std]
#![allow(non_camel_case_types)]

pub use nrf51_hal as hal;

pub use nb::*;

pub use crate::hal::nrf51::*;

use crate::hal::gpio::gpio::Parts;
use crate::hal::serial::*;

pub mod display;
pub mod led;

// FIXME: Rewrite as macro to prevent problems consuming parts of gpio
pub fn serial_port(
    gpio: Parts,
    uart: hal::nrf51::UART0,
    speed: BAUDRATE_A,
) -> (Tx<hal::nrf51::UART0>, Rx<hal::nrf51::UART0>) {
    /* Configure RX and TX pins accordingly */
    let tx = gpio.pin24.into_push_pull_output().into();
    let rx = gpio.pin25.into_floating_input().into();

    /* Set up serial port using the prepared pins */
    let serial = Serial::uart0(uart, tx, rx, speed);
    serial.split()
}
