//! On-board user LEDs

use crate::hal::{
    gpio::{Output, Pin, PushPull},
    prelude::*,
};

use crate::gpio::DisplayPins;

use embedded_hal::blocking::delay::DelayUs;

#[allow(clippy::upper_case_acronyms)]
pub(crate) type LED = Pin<Output<PushPull>>;

const DEFAULT_DELAY_MS: u32 = 2;
const LED_LAYOUT: [[(usize, usize); 5]; 5] = [
    [(0, 0), (1, 3), (0, 1), (1, 4), (0, 2)],
    [(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)],
    [(1, 1), (0, 8), (1, 2), (2, 8), (1, 0)],
    [(0, 7), (0, 6), (0, 5), (0, 4), (0, 3)],
    [(2, 2), (1, 6), (2, 0), (1, 5), (2, 1)],
];

/// Array of all the LEDs in the 5x5 display on the board
pub struct Display {
    delay_ms: u32,
    rows: [LED; 3],
    cols: [LED; 9],
}

impl Display {
    /// Initializes all the user LEDs
    #[allow(clippy::too_many_arguments)]
    pub fn new(pins: DisplayPins) -> Self {
        let mut retval = Display {
            delay_ms: DEFAULT_DELAY_MS,
            rows: [
                pins.row1.degrade(),
                pins.row2.degrade(),
                pins.row3.degrade(),
            ],
            cols: [
                pins.col1.degrade(),
                pins.col2.degrade(),
                pins.col3.degrade(),
                pins.col4.degrade(),
                pins.col5.degrade(),
                pins.col6.degrade(),
                pins.col7.degrade(),
                pins.col8.degrade(),
                pins.col9.degrade(),
            ],
        };
        // This is needed to reduce flickering on reset
        retval.clear();
        retval
    }

    /// Clear display
    pub fn clear(&mut self) {
        for row in &mut self.rows {
            row.set_low().ok();
        }
        for col in &mut self.cols {
            col.set_high().ok();
        }
    }

    /// Set delay, time spent on each matrix row, in ms
    pub fn set_delay_ms(&mut self, delay_ms: u32) {
        self.delay_ms = delay_ms;
    }

    /// Set refresh rate, time for matrix scan
    pub fn set_refresh_rate(&mut self, freq_hz: u32) {
        self.delay_ms = 1000 / freq_hz / 3;
    }

    /// Convert 5x5 display image to 3x9 matrix image
    pub fn display2matrix(led_display: [[u8; 5]; 5]) -> [[u8; 9]; 3] {
        let mut led_matrix: [[u8; 9]; 3] = [[0; 9]; 3];
        for (led_display_row, layout_row) in led_display.iter().zip(LED_LAYOUT.iter()) {
            for (led_display_val, layout_loc) in led_display_row.iter().zip(layout_row) {
                led_matrix[layout_loc.0][layout_loc.1] = *led_display_val;
            }
        }
        led_matrix
    }

    /// Display 5x5 display image for a given duration
    pub fn display<D: DelayUs<u32>>(
        &mut self,
        delay: &mut D,
        led_display: [[u8; 5]; 5],
        duration_ms: u32,
    ) {
        let led_matrix = Display::display2matrix(led_display);
        self.display_pre(delay, led_matrix, duration_ms);
    }

    /// Display 3x9 matrix image for a given duration
    pub fn display_pre<D: DelayUs<u32>>(
        &mut self,
        delay: &mut D,
        led_matrix: [[u8; 9]; 3],
        duration_ms: u32,
    ) {
        // TODO: something more intelligent with timers
        let loops = duration_ms / (self.rows.len() as u32 * self.delay_ms);
        for _ in 0..loops {
            for (row_line, led_matrix_row) in self.rows.iter_mut().zip(led_matrix.iter()) {
                row_line.set_high().ok();
                for (col_line, led_matrix_val) in self.cols.iter_mut().zip(led_matrix_row.iter()) {
                    // TODO : use value to set brightness
                    if *led_matrix_val > 0 {
                        col_line.set_low().ok();
                    }
                }
                delay.delay_us(self.delay_ms * 1000);
                for col_line in &mut self.cols {
                    col_line.set_high().ok();
                }
                row_line.set_low().ok();
            }
        }
    }
}
