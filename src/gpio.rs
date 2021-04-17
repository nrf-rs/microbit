//! Named GPIO pin types
//!
//! This module maps the GPIO pin names as described in the
//! [v1.5 schematic](https://github.com/bbcmicrobit/hardware/tree/master/V1.5).
//! Where appropriate the pins are restricted with the appropriate `MODE`
//! from `nrf-hal`.
#![allow(clippy::upper_case_acronyms)]
use crate::hal::gpio::{p0, Floating, Input, Output, PushPull};

/* GPIO pads */
pub type PAD1<MODE> = p0::P0_03<MODE>;
pub type PAD2<MODE> = p0::P0_02<MODE>;
pub type PAD3<MODE> = p0::P0_01<MODE>;

/* LED display */
pub type COL1 = p0::P0_04<Output<PushPull>>;
pub type COL2 = p0::P0_05<Output<PushPull>>;
pub type COL3 = p0::P0_06<Output<PushPull>>;
pub type COL4 = p0::P0_07<Output<PushPull>>;
pub type COL5 = p0::P0_08<Output<PushPull>>;
pub type COL6 = p0::P0_09<Output<PushPull>>;
pub type COL7 = p0::P0_10<Output<PushPull>>;
pub type COL8 = p0::P0_11<Output<PushPull>>;
pub type COL9 = p0::P0_12<Output<PushPull>>;

pub type ROW1 = p0::P0_13<Output<PushPull>>;
pub type ROW2 = p0::P0_14<Output<PushPull>>;
pub type ROW3 = p0::P0_15<Output<PushPull>>;

/* buttons */
pub type BTN_A = p0::P0_17<Input<Floating>>;
pub type BTN_B = p0::P0_26<Input<Floating>>;

/* spi */
pub type MOSI<MODE> = p0::P0_21<MODE>;
pub type MISO<MODE> = p0::P0_22<MODE>;
pub type SCK<MODE> = p0::P0_23<MODE>;

/* i2c */
pub type SCL = p0::P0_00<Input<Floating>>;
pub type SDA = p0::P0_30<Input<Floating>>;

/* uart */
pub type UART_TX = p0::P0_24<Output<PushPull>>;
pub type UART_RX = p0::P0_25<Input<Floating>>;

/* edge connector */
pub type EDGE01 = COL1;
pub type EDGE02<MODE> = PAD1<MODE>; // <- big pad 1
pub type EDGE03 = COL2;
pub type EDGE04 = BTN_A;
pub type EDGE05 = COL9;
pub type EDGE06 = COL8;
pub type EDGE07<MODE> = PAD2<MODE>; // <- big pad 2
pub type EDGE08<MODE> = p0::P0_18<MODE>;
pub type EDGE09 = COL7;
pub type EDGE10 = COL3;
pub type EDGE11 = BTN_B;
pub type EDGE12<MODE> = p0::P0_20<MODE>;
pub type EDGE13<MODE> = PAD3<MODE>; // <- big pad 3
pub type EDGE14<MODE> = SCK<MODE>;
pub type EDGE15<MODE> = MISO<MODE>;
pub type EDGE16<MODE> = MOSI<MODE>;
pub type EDGE17<MODE> = p0::P0_16<MODE>;
// EDGE18 -> +V
// EDGE19 -> +V
// EDGE20 -> +V
pub type EDGE21 = SCL;
pub type EDGE22 = SDA;
// EDGE23 -> GND
// EDGE24 -> GND
// EDGE25 -> GND
