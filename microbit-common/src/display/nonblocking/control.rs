//! Implementation of [`DisplayControl`] for the micro:bit's GPIO peripheral.
//!
//! This controls the micro:bit's 5×5 LED display.
//!
//! [`DisplayControl`]: tiny_led_matrix::DisplayControl

use tiny_led_matrix::DisplayControl;

use crate::{
    gpio::{NUM_COLS, NUM_ROWS},
    pac,
};

const fn pin_bits(pins: &[usize]) -> u32 {
    let mut i: usize = 0;
    let mut bits: u32 = 0;
    while i < pins.len() {
        bits |= 1 << pins[i];
        i += 1;
    }
    bits
}

#[cfg(feature = "v1")]
mod pins {
    use super::{NUM_COLS, NUM_ROWS};
    pub(super) const P0_COLS: [usize; NUM_COLS] = [4, 5, 6, 7, 8, 9, 10, 11, 12];
    pub(super) const P0_ROWS: [usize; NUM_ROWS] = [13, 14, 15];
}

#[cfg(feature = "v2")]
mod pins {
    use super::{NUM_COLS, NUM_ROWS};
    pub(super) const P0_COLS: [usize; NUM_COLS - 1] = [28, 11, 31, 30];
    pub(super) const P1_COLS: [usize; 1] = [5];

    pub(super) const P0_ROWS: [usize; NUM_ROWS] = [21, 22, 15, 24, 19];
}

const P0_COL_BITS: u32 = pin_bits(&pins::P0_COLS);
#[cfg(feature = "v2")]
const P1_COL_BITS: u32 = pin_bits(&pins::P1_COLS);

const P0_ROW_BITS: u32 = pin_bits(&pins::P0_ROWS);

#[cfg(feature = "v1")]
type P0 = pac::GPIO;

#[cfg(feature = "v2")]
type P0 = pac::P0;

#[cfg(feature = "v2")]
type P1 = pac::P1;

/// This implements the `DisplayControl` trait.
///
/// [`DisplayControl`]: tiny_led_matrix::DisplayControl
pub(crate) struct MicrobitGpio;

/// Returns the GPIO pin numbers corresponding to the columns in a Column
fn column_pins(mut cols: u32, px_cols: &[usize]) -> u32 {
    let mut result = 0u32;
    for &pin in px_cols.iter() {
        result |= (cols & 1) << pin;
        cols >>= 1;
    }
    result
}

#[cfg(feature = "v1")]
fn split_cols(cols: u32) -> (u32, u32) {
    (cols, 0u32)
}

#[cfg(feature = "v2")]
fn split_cols(cols: u32) -> (u32, u32) {
    // get all except col 2 (4th from least significant)
    let p0_cols = ((cols & 0b10000) >> 1) | (0b00111 & cols);
    // get col 4 (4th from least significant)
    let p1_cols = (cols & 0b01000) >> 3;
    (p0_cols, p1_cols)
}

/// Implementation of [`DisplayControl`] for the micro:bit's GPIO peripheral.
///
/// This controls the micro:bit's 5×5 LED display.
///
/// The `initialise_for display` implementation assumes the port is in the
/// state it would have after system reset.
///
/// [`DisplayControl`]: tiny_led_matrix::DisplayControl
impl DisplayControl for MicrobitGpio {
    fn initialise_for_display(&mut self) {
        unsafe {
            let p0 = &*P0::ptr();
            for ii in pins::P0_COLS.iter() {
                p0.pin_cnf[*ii].write(|w| w.dir().output());
            }
            for ii in pins::P0_ROWS.iter() {
                p0.pin_cnf[*ii].write(|w| w.dir().output());
            }

            // Set all p0 cols high.
            p0.outset
                .write(|w| w.bits(pins::P0_COLS.iter().map(|pin| 1 << pin).sum()));

            #[cfg(feature = "v2")]
            {
                let p1 = &*P1::ptr();
                for ii in pins::P1_COLS.iter() {
                    p1.pin_cnf[*ii].write(|w| w.dir().output());
                }
                // Set all p1 cols high.
                p1.outset
                    .write(|w| w.bits(pins::P1_COLS.iter().map(|pin| 1 << pin).sum()));
            }
        }
    }

    fn display_row_leds(&mut self, row: usize, cols: u32) {
        unsafe {
            let p0 = &*P0::ptr();

            #[allow(unused_variables)]
            let (p0cols, p1cols) = split_cols(cols);

            // To light an LED, we set the row bit and clear the col bit.
            let rows_to_set = 1 << pins::P0_ROWS[row];
            let rows_to_clear = P0_ROW_BITS ^ rows_to_set;

            #[cfg(feature = "v1")]
            {
                let cols_to_clear = column_pins(p0cols, &pins::P0_COLS);
                let cols_to_set = P0_COL_BITS ^ cols_to_clear;
                p0.outset.write(|w| w.bits(rows_to_set | cols_to_set));
                p0.outclr.write(|w| w.bits(rows_to_clear | cols_to_clear));
            }

            #[cfg(feature = "v2")]
            {
                let p1 = &*P1::ptr();
                let p0_cols_to_clear = column_pins(p0cols, &pins::P0_COLS);
                let p0_cols_to_set = P0_COL_BITS ^ p0_cols_to_clear;
                let p1_cols_to_clear = column_pins(p1cols, &pins::P1_COLS);
                let p1_cols_to_set = P1_COL_BITS ^ p1_cols_to_clear;
                // We do the row-clearing write first and the row-setting write last, so that
                // intermediate states never light LEDs which aren't lit in either the old or new state.
                p0.outclr
                    .write(|w| w.bits(rows_to_clear | p0_cols_to_clear));
                p1.outset.write(|w| w.bits(p1_cols_to_set));
                p1.outclr.write(|w| w.bits(p1_cols_to_clear));
                p0.outset.write(|w| w.bits(rows_to_set | p0_cols_to_set));
            }
        }
    }

    fn light_current_row_leds(&mut self, cols: u32) {
        unsafe {
            #[allow(unused_variables)]
            let (p0cols, p1cols) = split_cols(cols);
            let p0 = &*P0::ptr();
            p0.outclr
                .write(|w| w.bits(column_pins(p0cols, &pins::P0_COLS)));

            #[cfg(feature = "v2")]
            {
                let p1 = &*P1::ptr();
                p1.outclr
                    .write(|w| w.bits(column_pins(p1cols, &pins::P1_COLS)));
            }
        }
    }
}
