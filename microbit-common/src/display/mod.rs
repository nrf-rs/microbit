//! Support for the 5x5 LED display.
//!
//! There are two APIs for controlling the LED display, [`blocking`] and [`nonblocking`].
//! The `blocking` API is the simplest to get started with.
pub mod blocking;
pub mod nonblocking;
