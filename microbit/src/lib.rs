//! microbit contains everything required to get started with the use of Rust
//! to create firmwares for the fabulous [BBC micro:bit](https://microbit.org)
//! microcontroller board.
//!
//! This crate is for the original micro:bit (V1) pictured below on the left. If
//! your micro:bit looks like the one on the right you need the
//! [microbit-v2](https://crates.io/crates/microbit-v2) crate.
//!
//! [<img src="https://github.com/microbit-foundation/microbit-svg/raw/master/microbit-drawing-back-1-5.png" width="372px" height="300px">](https://github.com/microbit-foundation/microbit-svg/blob/master/microbit-drawing-back-1-5.png)
//! [<img src="https://github.com/microbit-foundation/microbit-svg/raw/master/microbit-drawing-back-2.png" width="372px" height="300px">](https://github.com/microbit-foundation/microbit-svg/blob/master/microbit-drawing-back-2.png)
#![doc(html_root_url = "https://docs.rs/microbit/0.9.0")]
#![no_std]
#![deny(missing_docs)]
#![allow(non_camel_case_types)]

pub use microbit_common::*;

/// Create a [Uart](hal::uart::Uart) client with the default pins
#[macro_export]
macro_rules! serial_port {
    ( $gpio:expr, $uart:expr, $speed:expr ) => {{
        use microbit::hal::{gpio::Level, uart};

        /* Configure RX and TX pins accordingly */
        let pins = uart::Pins {
            rxd: $gpio.p0_25.into_floating_input().degrade(),
            txd: $gpio.p0_24.into_push_pull_output(Level::Low).degrade(),
            cts: None,
            rts: None,
        };

        /* Set up serial port using the prepared pins */
        uart::Uart::new($uart, pins, uart::Parity::EXCLUDED, $speed)
    }};
}
