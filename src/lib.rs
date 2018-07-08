#![no_std]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(used))]
#![feature(const_fn)]
#![allow(non_camel_case_types)]

pub extern crate nrf51_hal as hal;

pub extern crate cortex_m;
pub extern crate nb;

extern crate cortex_m_rt;

pub use nb::*;

pub use cortex_m_rt::*;
pub use hal::nrf51;
pub use nrf51::interrupt::*;
pub use nrf51::*;

use hal::gpio::gpio::Parts;
use hal::serial::*;

pub mod led;

// FIXME: Rewrite as macro to prevent problems consuming parts of gpio
pub fn serial_port(
    gpio: Parts,
    uart: nrf51::UART0,
    speed: BAUDRATEW,
) -> (Tx<nrf51::UART0>, Rx<nrf51::UART0>) {
    /* Configure RX and TX pins accordingly */
    let tx = gpio.pin24.into_push_pull_output().downgrade();
    let rx = gpio.pin25.into_floating_input().downgrade();

    /* Set up serial port using the prepared pins */
    let serial = Serial::uart0(uart, tx, rx, speed);
    serial.split()
}
