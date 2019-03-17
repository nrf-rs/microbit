//! Implementation of [`DisplayTimer`] for the nrf51 `TIMER`s.
//!
//! [`DisplayTimer`]: tiny_led_matrix::DisplayTimer

use crate::hal::nrf51;
use tiny_led_matrix::DisplayTimer;

/// Wrapper for an nrf51 `TIMER` for passing to the display code.
///
/// This implements the [`DisplayTimer`] trait.
///
/// [`DisplayTimer`]: tiny_led_matrix::DisplayTimer
pub(crate) struct MicrobitTimer<'a>(pub &'a nrf51::timer0::RegisterBlock);

/// Checks whether the event for a CC register has been generated,
/// then clears the event register.
fn check_cc(timer: &nrf51::timer0::RegisterBlock, index: usize) -> bool {
    let event_reg = &timer.events_compare[index];
    let fired = event_reg.read().bits() != 0;
    if fired {
        event_reg.write(|w| unsafe { w.bits(0) })
    }
    fired
}

/// Implementation of DisplayTimer for one of the nrf51 `TIMER` peripherals.
///
/// The timer is set to 16-bit mode, using a 62.5kHz clock (16 µs ticks).
///
/// Uses CC0 for the primary cycle and CC1 for the secondary alarm. Uses the
/// CC0_CLEAR shortcut to implement the primary cycle.
///
/// The initialise_cycle() implementation assumes the timer is in the state it
/// would have after system reset.
///
/// check_primary() and check_secondary() take care of clearing the timer's
/// event registers.
impl DisplayTimer for MicrobitTimer<'_> {
    fn initialise_cycle(&mut self, ticks: u16) {
        let timer = &self.0;
        timer.prescaler.write(|w| unsafe { w.bits(8) });
        timer.cc[0].write(|w| unsafe { w.bits(ticks as u32) });
        timer.bitmode.write(|w| w.bitmode()._32bit());
        timer.shorts.write(|w| w.compare0_clear().enabled());
        timer.intenset.write(|w| w.compare0().set());
        timer.tasks_start.write(|w| unsafe { w.bits(1) });
    }

    fn enable_secondary(&mut self) {
        self.0.intenset.write(|w| w.compare1().set());
    }

    fn disable_secondary(&mut self) {
        self.0.intenclr.write(|w| w.compare1().clear());
    }

    fn program_secondary(&mut self, ticks: u16) {
        self.0.cc[1].write(|w| unsafe { w.bits(ticks as u32) });
    }

    fn check_primary(&mut self) -> bool {
        check_cc(&self.0, 0)
    }

    fn check_secondary(&mut self) -> bool {
        check_cc(&self.0, 1)
    }
}
