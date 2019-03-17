//! Implementation of [`DisplayControl`] for the micro:bit's GPIO peripheral.
//!
//! This controls the micro:bit's 5×5 LED display.
//!
//! [`DisplayControl`]: tiny_led_matrix::DisplayControl

use crate::hal::nrf51;
use tiny_led_matrix::DisplayControl;

const fn bit_range(lo: usize, count: usize) -> u32 {
    return ((1 << count) - 1) << lo;
}

pub(crate) const MATRIX_COLS: usize = 9;
const FIRST_COL_PIN: usize = 4;
const LAST_COL_PIN: usize = FIRST_COL_PIN + MATRIX_COLS - 1;
const COL_BITS: u32 = bit_range(FIRST_COL_PIN, MATRIX_COLS);

pub(crate) const MATRIX_ROWS: usize = 3;
const FIRST_ROW_PIN: usize = 13;
const LAST_ROW_PIN: usize = FIRST_ROW_PIN + MATRIX_ROWS - 1;
const ROW_BITS: u32 = bit_range(FIRST_ROW_PIN, MATRIX_ROWS);

/// Wrapper for `nrf51::GPIO` for passing to the display code.
///
/// This implements the `DisplayControl` trait.
///
/// [`DisplayControl`]: tiny_led_matrix::DisplayControl
pub struct MicrobitGpio<'a>(pub &'a nrf51::GPIO);

/// Returns the GPIO pin numbers corresponding to the columns in a ColumnSet.
fn column_pins(cols: u32) -> u32 {
    cols << FIRST_COL_PIN
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
        for ii in FIRST_COL_PIN..=LAST_COL_PIN {
            gpio.pin_cnf[ii].write(|w| w.dir().output());
        }
        for ii in FIRST_ROW_PIN..=LAST_ROW_PIN {
            gpio.pin_cnf[ii].write(|w| w.dir().output());
        }

        // Set all cols high.
        gpio.outset
            .write(|w| unsafe { w.bits((FIRST_COL_PIN..=LAST_COL_PIN).map(|pin| 1 << pin).sum()) });
    }

    fn display_row_leds(&mut self, row: usize, cols: u32) {
        let gpio = &self.0;
        // To light an LED, we set the row bit and clear the col bit.
        let rows_to_set = 1 << (FIRST_ROW_PIN + row);
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
