//! On-board user LEDs

use hal::delay::Delay;
use hal::gpio::gpio::PIN;
use hal::gpio::gpio::{PIN4, PIN5, PIN6, PIN7, PIN8, PIN9, PIN10, PIN11, PIN12, PIN13, PIN14, PIN15};
use hal::gpio::{Output, PushPull};
use hal::prelude::*;

type LED = PIN<Output<PushPull>>;


const BRIGHTNESS_BITS: u32 = 4;
// const BRIGHTNESS_MAX: u32 = 16; // 2^4
const BRIGHTNESS_DELAY_US: u32 = 8;
// const ROW_DELAY_MS: u32 = 2;
// const IAMGE_DELAY_MS: u32 = 3*ROW_DELAY_MS;
const LED_LAYOUT: [[(usize, usize); 5]; 5] = [
    [(0, 0), (1, 3), (0, 1), (1, 4), (0, 2)],
    [(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)],
    [(1, 1), (0, 8), (1, 2), (2, 8), (1, 0)],
    [(0, 7), (0, 6), (0, 5), (0, 4), (0, 3)],
    [(2, 2), (1, 6), (2, 0), (1, 5), (2, 1)],
];

/// Array of all the LEDs in the 5x5 display on the board
pub struct Display {
    delay_us: u32,
    rows: [LED; 3],
    cols: [LED; 9],
}

impl Display {
    /// Initializes all the user LEDs
    pub fn new(
        col1: PIN4<Output<PushPull>>,
        col2: PIN5<Output<PushPull>>,
        col3: PIN6<Output<PushPull>>,
        col4: PIN7<Output<PushPull>>,
        col5: PIN8<Output<PushPull>>,
        col6: PIN9<Output<PushPull>>,
        col7: PIN10<Output<PushPull>>,
        col8: PIN11<Output<PushPull>>,
        col9: PIN12<Output<PushPull>>,
        row1: PIN13<Output<PushPull>>,
        row2: PIN14<Output<PushPull>>,
        row3: PIN15<Output<PushPull>>,
    ) -> Self {
        let mut retval = Display {
            delay_us: BRIGHTNESS_DELAY_US,
            rows: [row1.downgrade(), row2.downgrade(), row3.downgrade()],
            cols: [
                col1.downgrade(), col2.downgrade(), col3.downgrade(),
                col4.downgrade(), col5.downgrade(), col6.downgrade(),
                col7.downgrade(), col8.downgrade(), col9.downgrade()
            ],
        };
        // This is needed to reduce flickering on reset
        retval.clear();
        retval
    }

    /// Clear display
    pub fn clear(&mut self) {
        for row in &mut self.rows {
            row.set_low();
        }
        for col in &mut self.cols {
            col.set_high();
        }
    }

    /// Set delay, time spent on each matrix row, in ms
    pub fn set_delay_ms(&mut self, delay_ms: u32) {
        self.delay_us = delay_ms * 1000;
    }

    /// Set refresh rate, time for matrix scan
    pub fn set_refresh_rate(&mut self, freq_hz: u32) {
        self.delay_us = 1_000_000 / freq_hz / 3;
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
    pub fn display(
        &mut self,
        delay: &mut Delay,
        led_display: [[u8; 5]; 5],
        duration_ms: u32
    ) {
        let led_matrix = Display::display2matrix(led_display);
        self.display_precalculated(delay, led_matrix, duration_ms, 0);
    }

    /// Display 5x5 display image for a given duration
    pub fn display_bright(
        &mut self,
        delay: &mut Delay,
        led_display: [[u8; 5]; 5],
        duration_ms: u32,
    ) {
        let min_trailing = led_display.iter().flatten().map(|x| x.trailing_zeros()).min().unwrap();
        let precision_bits = BRIGHTNESS_BITS - min_trailing;
        let led_matrix = Display::display2matrix(led_display);
        self.display_precalculated(delay, led_matrix, duration_ms, precision_bits);
    }

    /// Display 3x9 matrix image for a given duration
    pub fn display_precalculated(
        &mut self,
        delay: &mut Delay,
        led_matrix: [[u8; 9]; 3],
        duration_ms: u32,
        brightness_bits: u32,
    ) {
        // Number of brightness level loops
        let brightness_loops = 2_u32.pow(brightness_bits);
        // How much each loop is worth
        let brightness_factor = 2_u32.pow(BRIGHTNESS_BITS-brightness_bits);
        // Calculates how long to block for
        // e.g. If the duration_ms is 500ms (half a second)
        //      and self.delay_ms is 2ms (about 2ms per scan row),
        //      each refresh takes 3rows×2ms, so we need 500ms / (3×2ms) loops.
        // TODO: something more intelligent with timers
        let loops = (1000 * duration_ms) / 
                    (self.rows.len() as u32 * brightness_loops * self.delay_us);
        for _ in 0..loops {
            for brightness_loop in 0..brightness_loops {
                let brightness_threshold: u8 = (brightness_factor * brightness_loop) as u8;
                for (row_line, led_matrix_row) in self.rows.iter_mut().zip(led_matrix.iter()) {
                    row_line.set_high();
                    for (col_line, led_matrix_val) in self.cols.iter_mut().zip(led_matrix_row.iter()) {
                        if *led_matrix_val > brightness_threshold {
                            col_line.set_low();
                        }
                    }
                    delay.delay_us(brightness_loops * self.delay_us);
                    for col_line in &mut self.cols {
                        col_line.set_high();
                    }
                    row_line.set_low();
                }
            }
        }
    }
}
