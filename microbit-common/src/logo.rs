//! Capacitive touch sensing for the micro:bit v2 gold logo.
//!
//! The logo on the front of the micro:bit v2 is connected to pin `P1_04` and
//! acts as a capacitive touch sensor. This module provides [`Logo`], a small
//! wrapper that reports whether the logo is currently being touched.
//!
//! # How it works
//!
//! There is no dedicated touch peripheral involved. Instead, touch is detected
//! by timing how long the pin takes to charge:
//!
//! 1. The pin is driven low briefly to drain any charge from the pad.
//! 2. The pin is switched to a floating input, and the code counts how many
//!    microseconds elapse before the board's 10 MΩ pull-up charges the pad back
//!    to a logic high.
//!
//! A finger on the logo adds capacitance, so the pad takes noticeably longer to
//! charge. An untouched pad charges in roughly 15 µs; a touched pad takes
//! substantially longer. [`Logo::is_touched`] compares the measured charge time
//! against a threshold (with hysteresis to avoid flicker near the boundary) and
//! returns whether the logo is being touched.
//!
//! Because the measurement actively drives the pin and needs microsecond
//! delays, [`Logo::is_touched`] takes `&mut self` and a delay provider, unlike
//! the simple level reads used for the buttons.
//!
//! # Example
//!
//! ```no_run
//! use microbit::Board;
//! use microbit::hal::Timer;
//!
//! let board = Board::take().unwrap();
//! let mut timer = Timer::new(board.TIMER0);
//! let mut logo = microbit::logo::Logo::new(board.pins.p1_04);
//!
//! loop {
//!     if logo.is_touched(&mut timer) {
//!         // the logo is being touched
//!     }
//! }
//! ```
//!
//! Detecting "pressed" and "released" edge events can be done in user code by
//! remembering the previous result of [`Logo::is_touched`]; see the
//! `logo-touch` example.
//!
//! # Attribution
//!
//! The charge-timing approach used here is adapted from the MIT-licensed
//! `pdx-cs-rust-embedded/mb2-touch` demonstration by the Portland State
//! University CS Rust Embedded group.

use crate::hal::gpio::{p1::P1_04, Disconnected, Floating, Input, Level, Pin};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;

/// Microseconds the pad is driven low to drain it before each measurement.
const RESET_TIME_US: u32 = 10;

/// Ceiling on the charge-time count, in microseconds. The measurement loop
/// stops here so a stuck-low pin can never hang the caller.
const MAX_TICKS: u32 = 5_000;

/// Charge time (µs) above which an untouched logo becomes "touched".
const TOUCH_HIGH: u32 = 50;

/// Charge time (µs) below which a touched logo becomes "released". Keeping this
/// lower than [`TOUCH_HIGH`] gives hysteresis, which prevents the reported
/// state from flickering when the charge time hovers near the boundary.
const TOUCH_LOW: u32 = 30;

/// The capacitive touch logo on the front of the micro:bit v2.
///
/// Construct one from the logo pin (`board.pins.p1_04`) with [`Logo::new`], then
/// poll it with [`Logo::is_touched`]. See the [module documentation](self) for
/// details on how detection works.
pub struct Logo {
    // Held as an `Option` so the pin can be temporarily moved out to flip its
    // direction (output to charge, input to measure) on each measurement.
    pin: Option<Pin<Input<Floating>>>,
    touched: bool,
}

impl Logo {
    /// Create a [`Logo`] from the logo pin.
    ///
    /// The pin is taken in its default disconnected state, exactly as exposed by
    /// [`Board::pins`](crate::board::Board) as `pins.p1_04`.
    pub fn new(pin: P1_04<Disconnected>) -> Self {
        Self {
            pin: Some(pin.into_floating_input().degrade()),
            touched: false,
        }
    }

    /// Returns `true` while the logo is being touched.
    ///
    /// This performs one capacitive measurement, which drives the pin and uses
    /// `delay` for the required microsecond timing. The result is debounced with
    /// hysteresis, so a finger resting on the logo reports `true` consistently
    /// and a released logo reports `false` consistently.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use microbit::Board;
    /// # use microbit::hal::Timer;
    /// # let board = Board::take().unwrap();
    /// # let mut timer = Timer::new(board.TIMER0);
    /// let mut logo = microbit::logo::Logo::new(board.pins.p1_04);
    /// if logo.is_touched(&mut timer) {
    ///     // react to the touch
    /// }
    /// ```
    pub fn is_touched<D: DelayNs>(&mut self, delay: &mut D) -> bool {
        let charge_time = self.sense(delay);
        self.touched = if self.touched {
            charge_time > TOUCH_LOW
        } else {
            charge_time > TOUCH_HIGH
        };
        self.touched
    }

    /// Perform one raw measurement: drain the pad, then return the time in
    /// microseconds for it to charge back to a logic high (capped at
    /// [`MAX_TICKS`]). A larger value means more capacitance, i.e. a touch.
    ///
    /// The pin is moved out of `self.pin` to flip its direction and is always
    /// stored back before returning, so `self.pin` is `Some` between calls and
    /// this method never panics. If the pin were somehow absent, it reports `0`
    /// (untouched), which is the safe default.
    fn sense<D: DelayNs>(&mut self, delay: &mut D) -> u32 {
        let Some(pin) = self.pin.take() else {
            return 0;
        };

        // Drive low to drain the pad to 0 V.
        let out = pin.into_push_pull_output(Level::Low);
        delay.delay_us(RESET_TIME_US);

        // Switch to a floating input and time the charge back up to high.
        let mut input = out.into_floating_input();
        let mut count = 0u32;
        while count < MAX_TICKS && input.is_low().unwrap_or(false) {
            delay.delay_us(1);
            count += 1;
        }

        self.pin = Some(input);
        count
    }
}
