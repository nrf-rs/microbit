//! microbit contains everything required to get started with the use of Rust
//! to create firmwares for the fabulous [BBC micro:bit](https://microbit.org)
//! microcontroller board.
//!
//! This crate is for the new micro:bit (V2) pictured below on the right. If
//! your micro:bit looks like the one on the left you need the
//! [microbit](https://crates.io/crates/microbit) crate.
//!
//! [<img src="https://github.com/microbit-foundation/microbit-svg/raw/master/microbit-drawing-back-1-5.png" width="372px" height="300px">](https://github.com/microbit-foundation/microbit-svg/blob/master/microbit-drawing-back-1-5.png)
//! [<img src="https://github.com/microbit-foundation/microbit-svg/raw/master/microbit-drawing-back-2.png" width="372px" height="300px">](https://github.com/microbit-foundation/microbit-svg/blob/master/microbit-drawing-back-2.png)
#![doc(html_root_url = "https://docs.rs/microbit-v2/0.15.1")]
#![no_std]
#![deny(missing_docs)]
#![allow(non_camel_case_types)]

pub use microbit_common::*;
