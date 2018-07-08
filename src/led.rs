//! On-board user LEDs

use hal::delay::Delay;
use hal::gpio::gpio::PIN;
use hal::gpio::{Output, PushPull};
use hal::prelude::*;

type LED = PIN<Output<PushPull>>;

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
    pub fn new(
        row1: LED,
        row2: LED,
        row3: LED,
        col1: LED,
        col2: LED,
        col3: LED,
        col4: LED,
        col5: LED,
        col6: LED,
        col7: LED,
        col8: LED,
        col9: LED,
    ) -> Self {
        let mut retval = Display {
            delay_ms: DEFAULT_DELAY_MS,
            rows: [row1, row2, row3],
            cols: [col1, col2, col3, col4, col5, col6, col7, col8, col9],
        };
        // This is needed to reduce flickering on reset
        retval.clear();
        return retval;
    }

    /// Clear display
    pub fn clear(&mut self) {
        for row in self.rows.iter_mut() {
            row.set_low();
        }
        for col in self.cols.iter_mut() {
            col.set_high();
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
        return led_matrix;
    }

    /// Display 5x5 display image for a given duration
    pub fn display(&mut self, delay: &mut Delay, led_display: [[u8; 5]; 5], duration_ms: u32) {
        let led_matrix = Display::display2matrix(led_display);
        self.display_pre(delay, led_matrix, duration_ms);
    }

    /// Display 3x9 matrix image for a given duration
    pub fn display_pre(&mut self, delay: &mut Delay, led_matrix: [[u8; 9]; 3], duration_ms: u32) {
        // TODO: something more intelligent with timers
        let loops = duration_ms / (self.rows.len() as u32 * self.delay_ms);
        for _ in 0..loops {
            for (row_line, led_matrix_row) in self.rows.iter_mut().zip(led_matrix.iter()) {
                row_line.set_high();
                for (col_line, led_matrix_val) in self.cols.iter_mut().zip(led_matrix_row.iter()) {
                    // TODO : use value to set brightness
                    if *led_matrix_val > 0 {
                        col_line.set_low();
                    }
                }
                delay.delay_ms(self.delay_ms);
                for col_line in self.cols.iter_mut() {
                    col_line.set_high();
                }
                row_line.set_low();
            }
        }
    }
}
