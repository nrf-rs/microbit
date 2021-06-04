//! Implementation of [`Matrix`] and [`Frame`] for the micro:bit's LED display.
//!
//! This module describes the correspondence between the visible layout of
//! micro:bit's LEDs and the pins controlling them.
//!
//! [`Matrix`]: tiny_led_matrix::Matrix
//! [`Frame`]: tiny_led_matrix::Frame

use crate::gpio::{NUM_COLS, NUM_ROWS};
use tiny_led_matrix::{Frame, Matrix, RowPlan};

/// Implementation of [`Matrix`] for the microbit's LED display.
///
/// [`Matrix`]: tiny_led_matrix::Matrix
pub struct MicrobitMatrix();

/// Gives the LED (x, y) coordinates for a given pin row and column.
/// The origin is in the top-left.
#[cfg(feature = "v1")]
const MICROBIT_LED_LAYOUT: [[Option<(usize, usize)>; 3]; 9] = [
    [Some((0, 0)), Some((4, 2)), Some((2, 4))],
    [Some((2, 0)), Some((0, 2)), Some((4, 4))],
    [Some((4, 0)), Some((2, 2)), Some((0, 4))],
    [Some((4, 3)), Some((1, 0)), Some((0, 1))],
    [Some((3, 3)), Some((3, 0)), Some((1, 1))],
    [Some((2, 3)), Some((3, 4)), Some((2, 1))],
    [Some((1, 3)), Some((1, 4)), Some((3, 1))],
    [Some((0, 3)), None, Some((4, 1))],
    [Some((1, 2)), None, Some((3, 2))],
];

impl Matrix for MicrobitMatrix {
    /// The number of pins connected to LED columns (3).
    const MATRIX_COLS: usize = NUM_COLS;
    /// The number of pins connected to LED rows (9).
    const MATRIX_ROWS: usize = NUM_ROWS;
    /// The number of visible LED columns (5).
    const IMAGE_COLS: usize = 5;
    /// The number of visible LED rows (5).
    const IMAGE_ROWS: usize = 5;

    #[cfg(feature = "v1")]
    fn image_coordinates(col: usize, row: usize) -> Option<(usize, usize)> {
        MICROBIT_LED_LAYOUT[col][row]
    }

    #[cfg(feature = "v2")]
    fn image_coordinates(col: usize, row: usize) -> Option<(usize, usize)> {
        Some((col, row))
    }
}

/// A 'Compiled' representation of a 5×5 image to be displayed.
///
/// Use the [`.set()`](`Frame::set`) method to store an image (something
/// implementing [`Render`]) in the frame.
///
/// Note you'll have to `use microbit::display::Frame` to make `set()`
/// available.
///
/// [`Frame`]: tiny_led_matrix::Frame
/// [`Render`]: tiny_led_matrix::Render
#[derive(Copy, Clone, Debug)]
pub struct MicrobitFrame([RowPlan; MicrobitFrame::ROWS]);

impl MicrobitFrame {
    /// Returns a new frame, initially blank.
    pub const fn default() -> MicrobitFrame {
        MicrobitFrame([RowPlan::default(); MicrobitFrame::ROWS])
    }
}

impl Default for MicrobitFrame {
    /// Returns a new frame, initially blank.
    fn default() -> MicrobitFrame {
        MicrobitFrame::default()
    }
}

impl Frame for MicrobitFrame {
    type Mtx = MicrobitMatrix;

    fn row_plan(&self, row: usize) -> &RowPlan {
        &self.0[row]
    }

    fn row_plan_mut(&mut self, row: usize) -> &mut RowPlan {
        &mut self.0[row]
    }
}
