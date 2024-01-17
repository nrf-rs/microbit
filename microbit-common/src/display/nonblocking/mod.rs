//! Non-blocking support for the 5×5 LED display.
//!
//! Together with [`tiny-led-matrix`](tiny_led_matrix), this module provides:
//! - support for driving the LED display from a timer interrupt
//! - ten levels of brightness for each LED
//! - simple 5×5 greyscale and black-and-white image types.
//!
//! The module doesn't define interrupt handlers directly; instead it provides
//! a function to be called from a timer interrupt. It knows how to program
//! one of the micro:bit's timers to provide that interrupt.
//!
//! ## Example
//!
//! This shows general usage but is not a working example.
//! For a working exaple see
//! [`display_nonblocking`](https://github.com/nrf-rs/microbit/tree/main/examples/display-nonblocking).
//!
//! ```no_run
//! # use microbit_common as microbit;
//! use microbit::{
//!     Board,
//!     hal,
//!     display::nonblocking::{Display, GreyscaleImage},
//! };
//! use embedded_hal::delay::DelayNs;
//!
//! let board = Board::take().unwrap();
//!
//! let mut display = Display::new(board.TIMER1, board.display_pins);
//!
//! // in your main function
//! {
//!     let mut timer2 = hal::Timer::new(board.TIMER0);
//!     loop {
//!         display.show(&GreyscaleImage::new(&[
//!             [0, 7, 0, 7, 0],
//!             [7, 0, 7, 0, 7],
//!             [7, 0, 0, 0, 7],
//!             [0, 7, 0, 7, 0],
//!             [0, 0, 7, 0, 0],
//!         ]));
//!         timer2.delay_ms(1000);
//!
//!         display.clear();
//!         timer2.delay_ms(1000);
//!     }
//! }
//!
//! // in a timer interrupt
//! {
//!     display.handle_display_event();
//! }
//! ```
//!
//! ## Coordinate system
//!
//! The LEDs are identified using (x,y) coordinates as follows:
//!
//! ```text
//! (0,0) ... (4,0)
//!  ...  ...  ...
//! (0,4) ... (4,4)
//! ```
//!
//! where the 'bottom' (x,4) of the board is the edge connector.
//!
//! ## Greyscale model
//!
//! LED brightness levels are described using a scale from 0 (off) to 9
//! (brightest) inclusive.
//!
//! These are converted to time slices using the same timings as used by the
//! [micro:bit MicroPython port][micropython] (this is different to the 0 to
//! 255 scale used by the [micro:bit runtime][dal]).
//!
//! The time slice for each level above 1 is approximately 1.9× the slice for
//! the previous level.
//!
#![cfg_attr(
    feature = "v1",
    doc = "An LED with brightness 9 is lit for one third of the time (because internally there are three 'rows' of LEDs which have to be addressed one at a time)."
)]
#![cfg_attr(
    feature = "v2",
    doc = "An LED with brightness 9 is lit for one fifth of the time."
)]
//!
//! ## Images
//!
//! An image is a type that implements the [`tiny_led_matrix::Render`] trait. Two image types are provided:
//! - [`GreyscaleImage`](image::GreyscaleImage), allowing all 9 levels (using one byte for each LED)
//! - [`BitImage`](image::BitImage), allowing only 'on' and 'off' (using five bytes)
//!
//! ## Display
//!
//! A [`Display`] instance controls the LEDs and programs a timer. There
//! should normally be a single `Display` instance in the program. It is a wrapper
//! around [`tiny_led_matrix::Display`] to expose an API similar to the blocking API.
//!
//! ## Frames
//!
//! Internally types implementing [`Render`](tiny_led_matrix::Render) aren't used directly with the [`Display`];
//! instead they're used to update a [`MicrobitFrame`] instance which is in
//! turn passed to the `tiny_led_matrix::Display`.
//!
//! A `MicrobitFrame` instance is a 'compiled' representation of a 5×5
//! greyscale image, in a form that's more directly usable by the display
//! code.
//!
//! This is exposed in the public API so that you can construct the
//! `MicrobitFrame` representation in code running at a low priority. Then
//! only [`Display::show_frame()`] has to be called in code that can't be
//! interrupted by the display timer.
//!
//! ## Timer integration
//!
//! The [`Display`] expects to control a single timer. It can use the
//! micro:bit's `TIMER0`, `TIMER1`, or `TIMER2`.
//!
//! For the micro:bit v1 this uses a 6ms period to light each of the three
//! internal LED rows, so that the entire display is updated every 18ms.
//!
//! For the micro:bit v2 this uses a 3ms period to light each of the five
//! internal LED rows, so that the entire display is updated every 15ms.
//!
//! When rendering greyscale images, the `Display` requests extra interrupts
//! within each 6ms or 3ms period. It only requests interrupts for the
//! greyscale levels which are actually required for what's currently being
//! displayed.
//!
//! ### Technical details
//!
//! The timer is set to 16-bit mode, using a 62.5kHz or 135Khz clock (16 µs or
//! 8µs ticks). It resets every 375 ticks.
//!
//! ## Usage
//!
//! Choose a timer to drive the display from (`TIMER0`, `TIMER1`, or `TIMER2`).
//!
//! When your program starts:
//! - create a [`Display`] struct passing the timer and
//! [`gpio::DisplayPins`](crate::gpio::DisplayPins) to [`Display::new()`].
//!
//! In an interrupt handler for the timer call [`.handle_display_event()`](Display::handle_display_event)
//!
//! To change what's displayed; pass an image ([`GreyscaleImage`] or [`BitImage`]) to [`Display::show`].
//!
//! You can call `show()` at any time, so long as you're not interrupting, or interruptable by,
//! [`Display::handle_display_event()`].
//!
//! See [`display_rtic`](https://github.com/nrf-rs/microbit/blob/master/examples/display_rtic) or
//! [`display_nonblocking`](https://github.com/nrf-rs/microbit/blob/master/examples/display_nonblocking)
//! example for a complete working example.
//!
//! [dal]: https://lancaster-university.github.io/microbit-docs/
//! [micropython]: https://microbit-micropython.readthedocs.io/

use tiny_led_matrix;
#[doc(no_inline)]
pub use tiny_led_matrix::{Frame, MAX_BRIGHTNESS};

mod control;
mod image;
mod matrix;
mod timer;

pub use image::{BitImage, GreyscaleImage};
pub use matrix::MicrobitFrame;
use timer::MicrobitDisplayTimer;

use crate::{gpio::DisplayPins, hal::timer::Instance};

use control::MicrobitGpio;

/// Non-blocking interface to the on board 5x5 LED display
pub struct Display<T: Instance> {
    display: tiny_led_matrix::Display<MicrobitFrame>,
    timer: MicrobitDisplayTimer<T>,
    pins: DisplayPins,
    frame: MicrobitFrame,
}

impl<T: Instance> Display<T> {
    /// Create and initialise the display driver
    ///
    /// [`DisplayPins`] can be used from [`Board::display_pins`](crate::Board::display_pins)
    /// or the [`display_pins!`](crate::display_pins) macro can be used is manually.
    pub fn new(timer: T, pins: DisplayPins) -> Self {
        let mut display = Self {
            display: tiny_led_matrix::Display::new(),
            timer: MicrobitDisplayTimer::new(timer),
            pins,
            frame: MicrobitFrame::default(),
        };
        display.initialise();
        display
    }

    /// Release the timer and pins
    pub fn free(self) -> (T, DisplayPins) {
        (self.timer.free(), self.pins)
    }

    /// Initialise the display
    ///
    /// This is usually called immediately after creating the display driver.
    /// It does not need to be called in a critical section.
    fn initialise(&mut self) {
        tiny_led_matrix::initialise_control(&mut MicrobitGpio {});
        tiny_led_matrix::initialise_timer(&mut self.timer);
    }

    /// Update the LED display and timer state
    ///
    /// Call this in an interrupt handler for the timer you're using. This method
    /// takes care of updating the LED display and clearing the timer's event registers
    ///
    /// This may be called at any time, so long as the code calling it is not interrupting, or
    /// interruptable by `tiny_led_matrix::Display::handle_event()`. Within safe code, the borrow
    /// checker ensures that this requirement is fulfilled. When writing unsafe code, this method
    /// should be called from within a [critical
    /// section](https://docs.rs/cortex-m/0.7.2/cortex_m/interrupt/fn.free.html).
    pub fn handle_display_event(&mut self) {
        self.display
            .handle_event(&mut self.timer, &mut MicrobitGpio {});
    }

    /// Show a new image
    ///
    /// This may be called at any time, so long as the code calling it is not interrupting, or
    /// interruptable by `tiny_led_matrix::Display::handle_event()`. Within safe code, the borrow
    /// checker ensures that this requirement is fulfilled. When writing unsafe code, this method
    /// should be called from within a [critical
    /// section](https://docs.rs/cortex-m/0.7.2/cortex_m/interrupt/fn.free.html).
    ///
    /// ## Example
    ///
    /// ```ignore
    /// display.show(&GreyscaleImage::new(&[
    ///     [0, 7, 0, 7, 0],
    ///     [7, 0, 7, 0, 7],
    ///     [7, 0, 0, 0, 7],
    ///     [0, 7, 0, 7, 0],
    ///     [0, 0, 7, 0, 0],
    /// ]));
    /// ```
    pub fn show<R: tiny_led_matrix::Render>(&mut self, image: &R) {
        self.frame.set(image);
        self.display.set_frame(&self.frame);
    }

    /// Clear the display
    ///
    /// This may be called at any time, so long as the code calling it is not interrupting, or
    /// interruptable by `tiny_led_matrix::Display::handle_event()`. Within safe code, the borrow
    /// checker ensures that this requirement is fulfilled. When writing unsafe code, this method
    /// should be called from within a [critical
    /// section](https://docs.rs/cortex-m/0.7.2/cortex_m/interrupt/fn.free.html).
    pub fn clear(&mut self) {
        self.display.set_frame(&MicrobitFrame::default());
    }

    /// Show a new frame
    ///
    /// This is similar to [`show`](Display::show) but accepts a [`MicrobitFrame`] instead.
    /// This may be useful if performance is a concern as calling `set` on the frame
    /// can be done outside the critical section.
    ///
    /// This may be called at any time, so long as the code calling it is not interrupting, or
    /// interruptable by `tiny_led_matrix::Display::handle_event()`. Within safe code, the borrow
    /// checker ensures that this requirement is fulfilled. When writing unsafe code, this method
    /// should be called from within a [critical
    /// section](https://docs.rs/cortex-m/0.7.2/cortex_m/interrupt/fn.free.html).
    ///
    /// ## Example
    ///
    /// ```ignore
    /// FRAME = MicrobitFrame::default();
    /// FRAME.set(&GreyscaleImage::new(&[
    ///     [0, 7, 0, 7, 0],
    ///     [7, 0, 7, 0, 7],
    ///     [7, 0, 0, 0, 7],
    ///     [0, 7, 0, 7, 0],
    ///     [0, 0, 7, 0, 0],
    /// ]));
    ///
    /// // only this needs to be in a critical section
    /// display.show_frame(&FRAME);
    /// ```
    pub fn show_frame(&mut self, frame: &MicrobitFrame) {
        self.display.set_frame(frame);
    }
}
