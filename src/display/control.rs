//! Implementation of [`DisplayControl`] for the micro:bit's GPIO peripheral.
//!
//! This controls the micro:bit's 5×5 LED display.
//!
//! [`DisplayControl`]: tiny_led_matrix::DisplayControl

use crate::pac;
use tiny_led_matrix::DisplayControl;

const fn pin_bits(pins: &[usize]) -> u32 {
    let mut i: usize = 0;
    let mut bits: u32 = 0;
    while i < pins.len() {
        bits |= 1 << pins[i];
        i += 1;
    }
    bits
}

pub(crate) const MATRIX_COLS: usize = 9;
const COLS: [usize; MATRIX_COLS] = [4, 5, 6, 7, 8, 9, 10, 11, 12];
const COL_BITS: u32 = pin_bits(&COLS);

pub(crate) const MATRIX_ROWS: usize = 3;
const ROWS: [usize; MATRIX_ROWS] = [13, 14, 15];
const ROW_BITS: u32 = pin_bits(&ROWS);

/// Wrapper for `nrf51::GPIO` for passing to the display code.
///
/// This implements the `DisplayControl` trait.
///
/// [`DisplayControl`]: tiny_led_matrix::DisplayControl
pub(crate) struct MicrobitGpio<'a>(pub &'a pac::GPIO);

/// Returns the GPIO pin numbers corresponding to the columns in a Columnt et.
fn column_pins(mut cols: u32) -> u32 {
    let mut result = 0u32;
    for pin in COLS.iter() {
        result |= (cols & 1) << pin;
        cols >>= 1;
    }
    result
}

/// Implementation of [`DisplayControl`] for the micro:bit's GPIO peripheral.
///
/// This controls the micro:bit's 5×5 LED display.
///
/// The `initialise_for display` implementation assumes the port is in the
/// state it would have after system reset.
///
/// [`DisplayControl`]: tiny_led_matrix::DisplayControl
impl DisplayControl for MicrobitGpio<'_> {
    fn initialise_for_display(&mut self) {
        let gpio = &self.0;
        for ii in COLS.iter() {
            gpio.pin_cnf[*ii].write(|w| w.dir().output());
        }
        for ii in ROWS.iter() {
            gpio.pin_cnf[*ii].write(|w| w.dir().output());
        }

        // Set all cols high.
        gpio.outset
            .write(|w| unsafe { w.bits(COLS.iter().map(|pin| 1 << pin).sum()) });
    }

    fn display_row_leds(&mut self, row: usize, cols: u32) {
        let gpio = &self.0;
        // To light an LED, we set the row bit and clear the col bit.
        let rows_to_set = 1 << ROWS[row];
        let rows_to_clear = ROW_BITS ^ rows_to_set;

        let cols_to_clear = column_pins(cols);
        let cols_to_set = COL_BITS ^ cols_to_clear;

        gpio.outset
            .write(|w| unsafe { w.bits(rows_to_set | cols_to_set) });
        gpio.outclr
            .write(|w| unsafe { w.bits(rows_to_clear | cols_to_clear) });
    }

    fn light_current_row_leds(&mut self, cols: u32) {
        let gpio = &self.0;
        gpio.outclr.write(|w| unsafe { w.bits(column_pins(cols)) });
    }
}
