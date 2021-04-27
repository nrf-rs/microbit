//! Named GPIO pin types
//!
//! This module maps the GPIO pin names as described in the
//! [Pins and Signals section of the micro:bit site](https://tech.microbit.org/hardware/edgeconnector/#pins-and-signals)
//! Where appropriate the pins are restricted with the appropriate `MODE`
//! from `nrf-hal`.
#[cfg(feature = "v1")]
pub use crate::v1::gpio::*;

#[cfg(feature = "v2")]
pub use crate::v2::gpio::*;
