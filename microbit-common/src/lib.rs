//! microbit contains everything required to get started with the use of Rust
//! to create firmwares for the fabulous [BBC micro:bit](https://microbit.org)
//! microcontroller board.
#![doc(html_root_url = "https://docs.rs/microbit-common/0.13.0")]
#![no_std]
#![deny(missing_docs)]
#![allow(non_camel_case_types)]

#[cfg(all(feature = "v1", feature = "v2"))]
compile_error!("canot build for microbit v1 and v2 at the same time");

pub mod adc;
pub mod board;
pub mod display;
pub mod gpio;

pub use embassy_nrf as hal;
pub use embassy_time as time;

pub use board::Board;

#[cfg(feature = "v1")]
mod v1;

#[cfg(feature = "v2")]
mod v2;
