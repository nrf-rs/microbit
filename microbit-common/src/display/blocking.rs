//! Blocking support for the 5x5 LED display.
//!
//! This module provides a simple blocking interface
//! to the on board 5x5 LED display. If you need a more sophisticated
//! or non-blocking interface use the [`display::nonblocking`](crate::display::nonblocking) module.
//!
//! # Example
//!
//! ```no_run
//! # use microbit_common as microbit;
//! # use microbit::{
//! #     Board,
//! #     hal,
//! #     display::blocking::Display,
//! # };
//! # use embedded_hal::delay::DelayNs;
//! // take the board
//! let board = Board::take().unwrap();
//! // make a timer
//! let mut timer = hal::Timer::new(board.TIMER0);
//! // create the Display
//! let mut display = Display::new(board.display_pins);
//! // and light up some LEDs
//! let heart = [
//!     [0, 1, 0, 1, 0],
//!     [1, 0, 1, 0, 1],
//!     [1, 0, 0, 0, 1],
//!     [0, 1, 0, 1, 0],
//!     [0, 0, 1, 0, 0],
//! ];
//! loop {
//!     display.show(&mut timer, heart, 1000);
//!     display.clear();
//!     timer.delay_ms(250);
//! }
//! ```
//!
//! The coordiante system is oriented so the 'bottom' (x,4) row is the edge with the edge
//! connector. That means that
//!
//! ```no_run
//! # use microbit_common as microbit;
//! # use microbit::{
//! #     Board,
//! #     hal,
//! #     display::blocking::Display,
//! # };
//! # let board = Board::take().unwrap();
//! # let mut timer = hal::Timer::new(board.TIMER0);
//! # let mut display = Display::new(board.display_pins);
//! display.show(
//!    &mut timer,
//!    [
//!        [0, 0, 1, 0, 0],
//!        [0, 1, 1, 1, 0],
//!        [1, 0, 1, 0, 1],
//!        [0, 0, 1, 0, 0],
//!        [0, 0, 1, 0, 0],
//!    ],
//!    1000,
//!);
//! ```
//! Will display an arrow pointing towards the boards usb port.
//!
//! For a working example [`examples/display-blocking`](https://github.com/nrf-rs/microbit/tree/main/examples/display-blocking)
use crate::gpio::{DisplayPins, NUM_COLS, NUM_ROWS};
use crate::hal::gpio::{Output, Pin, PushPull};
use embedded_hal::{delay::DelayNs, digital::OutputPin};

#[allow(clippy::upper_case_acronyms)]
pub(crate) type LED = Pin<Output<PushPull>>;

const DEFAULT_DELAY_MS: u32 = 2;
#[cfg(feature = "v1")]
const LED_LAYOUT: [[(usize, usize); 5]; 5] = [
    [(0, 0), (1, 3), (0, 1), (1, 4), (0, 2)],
    [(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)],
    [(1, 1), (0, 8), (1, 2), (2, 8), (1, 0)],
    [(0, 7), (0, 6), (0, 5), (0, 4), (0, 3)],
    [(2, 2), (1, 6), (2, 0), (1, 5), (2, 1)],
];

/// Blocking interface to the on board LED display
pub struct Display {
    delay_ms: u32,
    rows: [LED; NUM_ROWS],
    cols: [LED; NUM_COLS],
}

impl Display {
    /// Create and initialise the display driver
    ///
    /// The [`display_pins!`](crate::display_pins) macro can be used
    /// to create [`DisplayPins`].
    pub fn new(pins: DisplayPins) -> Self {
        let (cols, rows) = pins.degrade();
        Display {
            delay_ms: DEFAULT_DELAY_MS,
            rows,
            cols,
        }
    }

    /// Clear the display
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
        self.delay_ms = 1000 / freq_hz / (NUM_ROWS as u32);
    }

    /// Convert 5x5 image to 3x9 matrix
    ///
    /// The pins are represented as a [3x9 matrix on the micro:bit
    /// V1](https://tech.microbit.org/hardware/1-5-revision/#display).
    #[cfg(feature = "v1")]
    fn image2matrix(led_image: [[u8; 5]; 5]) -> [[u8; 9]; 3] {
        let mut led_matrix: [[u8; 9]; 3] = [[0; 9]; 3];
        for (led_image_row, layout_row) in led_image.iter().zip(LED_LAYOUT.iter()) {
            for (led_image_val, layout_loc) in led_image_row.iter().zip(layout_row) {
                led_matrix[layout_loc.0][layout_loc.1] = *led_image_val;
            }
        }
        led_matrix
    }

    /// Display 5x5 image for a given duration
    pub fn show<D: DelayNs>(&mut self, delay: &mut D, led_display: [[u8; 5]; 5], duration_ms: u32) {
        #[cfg(feature = "v1")]
        {
            let led_matrix = Display::image2matrix(led_display);
            self.show_inner(delay, led_matrix, duration_ms);
        }
        #[cfg(feature = "v2")]
        self.show_inner(delay, led_display, duration_ms);
    }

    /// Display matrix image for a given duration (3x9 for V1 micro:bit)
    ///
    /// The pins are represented as a [3x9 matrix on the micro:bit
    /// V1](https://tech.microbit.org/hardware/1-5-revision/#display).
    fn show_inner<D: DelayNs>(
        &mut self,
        delay: &mut D,
        led_matrix: [[u8; NUM_COLS]; NUM_ROWS],
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
