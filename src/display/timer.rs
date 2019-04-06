//! Implementation of [`DisplayTimer`] for the nRF51 `TIMER`s.
//!
//! [`DisplayTimer`]: tiny_led_matrix::DisplayTimer

use tiny_led_matrix::DisplayTimer;

use crate::hal::hi_res_timer::{As16BitTimer, HiResTimer, Nrf51Timer, TimerCc, TimerFrequency};

/// A TIMER peripheral programmed to manage the display.
///
/// `MicrobitDisplayTimer` instances implement the [`DisplayTimer`] trait.
///
/// The timer is set to 16-bit mode, using a 62.5kHz clock (16 µs ticks).
/// The primary cycle takes 6ms.
///
/// Uses CC0 for the primary cycle and CC1 for the secondary alarm. Uses the
/// CC0_CLEAR shortcut to implement the primary cycle.
///
/// [`DisplayTimer`]: tiny_led_matrix::DisplayTimer
pub struct MicrobitDisplayTimer<T: Nrf51Timer>(HiResTimer<T, u16>);

impl<T: As16BitTimer> MicrobitDisplayTimer<T> {
    /// Returns a new `MicrobitDisplayTimer` wrapping the passed TIMER.
    ///
    /// Takes ownership of the TIMER peripheral.
    pub fn new(timer: T) -> MicrobitDisplayTimer<T> {
        MicrobitDisplayTimer(timer.as_16bit_timer())
    }

    /// Gives the underlying `nrf51::TIMER`*n* instance back.
    pub fn free(self) -> T {
        self.0.free()
    }
}

impl<T: Nrf51Timer> DisplayTimer for MicrobitDisplayTimer<T> {
    fn initialise_cycle(&mut self, ticks: u16) {
        self.0.set_frequency(TimerFrequency::Freq62500Hz);
        self.0.set_compare_register(TimerCc::CC0, ticks);
        self.0.enable_auto_clear(TimerCc::CC0);
        self.0.enable_compare_interrupt(TimerCc::CC0);
        self.0.start();
    }

    fn enable_secondary(&mut self) {
        self.0.enable_compare_interrupt(TimerCc::CC1);
    }

    fn disable_secondary(&mut self) {
        self.0.disable_compare_interrupt(TimerCc::CC1);
    }

    fn program_secondary(&mut self, ticks: u16) {
        self.0.set_compare_register(TimerCc::CC1, ticks);
    }

    fn check_primary(&mut self) -> bool {
        self.0.poll_compare_event(TimerCc::CC0)
    }

    fn check_secondary(&mut self) -> bool {
        self.0.poll_compare_event(TimerCc::CC1)
    }
}
